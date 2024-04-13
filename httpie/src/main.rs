use anyhow::Ok;
use clap::Parser;
use clap::AppSettings;
use std::{collections::HashMap, str::FromStr};
use anyhow::{anyhow, Result};
use reqwest::{header, Client, Response, Url};
use colored::*;
use mime::Mime;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

/// CLI 的主入口，包含若干个子命令
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "pain")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opt {
    #[clap(subcommand)]
    command: Command,
}

/// 子命令分别对应不同的 HTTP 方法
#[derive(Parser, Debug)]
enum Command {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,

    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

/// 当实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (items.next().ok_or_else(err)?).to_string(),
            v: (items.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// 因为 KvPair 实现了 FromStr，可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

/// 处理 get 子命令
async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    Ok(print_response(res).await?)
}

/// 处理 post 子命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();

    for kv in args.body.iter() {
        body.insert(&kv.k, &kv.v);
    }

    let res = client.post(&args.url).json(&body).send().await?;
    Ok(print_response(res).await?)
}

fn print_status(res: &Response) {
    let status = format!("{:?} {}", res.version(), res.status()).blue();
    println!("{}\n", status);
}

fn print_headers(res: &Response) {
    for (name, value) in res.headers() {
        println!("{}: {:?}", name.to_string().green(), value)
    }

    println!("\n")
}

/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            print_syntect(body, "json")
            // println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        }
        Some(v) if v == mime::TEXT_HTML => {
            print_syntect(body, "html")
            // println!("{}", htmlxf::pretty_print(body).unwrap().cyan());
        }
        _ => println!("{}", body)
    }
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(res: &Response) -> Option<Mime> {
    res.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

/// 打印整个响应
async fn print_response(res: Response) -> Result<()> {
    print_status(&res);
    print_headers(&res);

    let mime = get_content_type(&res);
    let body = res.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn print_syntect(s: &str, ext: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

/// 程序的入口函数，因为在 HTTP 请求时我们使用了异步处理，所以这里引入 tokio
///
/// ./httpie post http://httpbin.org/post a=1 b=2
#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERD-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust fire".parse()?);
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opt.command {
        Command::Get(ref args) => get(client, args).await?,
        Command::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into(),
            }
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into(),
            }
        );
    }
}