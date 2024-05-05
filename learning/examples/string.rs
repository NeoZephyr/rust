use std::net::IpAddr;

fn main() {
    // escape();

    str_case1();
    str_case2();
    str_case3();
    byte_case1();
    byte_case2();
    convert();

    parse();
}

fn str_case1() {
    // "Hello world" -> 字符串的字面量，存放在静态数据区

    let s1 = "Hello world";

    // 指向静态数据区中的字符串的切片引用
    let s2: &'static str = s1;

    // 将静态数据区中的字符串字面量拷贝了一份到堆内存中
    let s3 = s1.to_string();

    // s4 就是对 s3 的不可变引用
    let s4 = &s3;

    // s5 是对 s3 的切片引用，类型是 &str
    // 切片是一块连续内存的某种视图，它可以提取目标对象的全部或一部分
    let s5 = &s3[..];
    let s6 = &s5[..5];
    let s7 = &s4[..];

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);
    println!("s4 = {}", s4);
    println!("s5 = {}", s5);
    println!("s6 = {}", s6);
    println!("s7 = {}", s7);
}

fn str_case2() {
    let s1 = "Hello world";
    let s2 = s1.to_string();
    let s3 = &s2[..];
    let s4 = s3.to_string();
    let s5 = s4.as_str();
    let s6 = s5.to_owned();

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);
    println!("s4 = {}", s4);
    println!("s5 = {}", s5);
    println!("s6 = {}", s6);
}

fn str_case3() {
    let s = String::from("牛顿万有引力");

    // char 用于存放 unicode 单个字符的类型
    // char 就是占用 4 个字节
    let cv1: Vec<char> = s.chars().collect();

    // 通过 Deref 的自动转换机制（解引用），将 String 转换为对应的 str 切片
    let cv2: Vec<char> = s.as_str().chars().collect();

    println!("cv1 = {:?}", cv1);
    println!("cv2 = {:?}", cv2);

    for c in s.chars() {
        println!("{c}")
    }

    for b in s.bytes() {
        println!("{b}")
    }
}

fn byte_case1() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v2 = &v1[0..5];

    let v3 = v2.to_vec();
    let v4 = v3.as_slice();

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v3 = {:?}", v3);
    println!("v4 = {:?}", v4);
}

fn byte_case2() {
    let s1 = String::from("hello");
    let s2 = s1.as_str();

    let v1 = s1.as_bytes();
    let v2 = s2.as_bytes();

    let s3 = std::str::from_utf8(v1);
    let v4 = String::from_utf8(v1.to_vec());

    println!("s1 = {:?}", s1);
    println!("s2 = {:?}", s2);

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    println!("s3 = {:?}", s3);
    println!("v4 = {:?}", v4);
}

fn convert() {
    let s1 = String::from("hello");
    let s2 = s1.as_str();
    foo1(&s1);
    // foo1(s2);

    // &String -> &str 隐式转换
    foo2(&s1);
    foo2(s2);

    let v1 = s1.as_bytes();
    let v2 = v1.to_vec();

    // bar1(v1);
    bar1(&v2);
    bar2(v1);

    // &Vec<u8> -> &[u8] 隐式转换
    bar2(&v2)
}

fn parse() {
    let n1 = "10".parse::<u32>().unwrap();
    let n2 = "3.14".parse::<f32>().unwrap();

    println!("n1 = {}", n1);
    println!("n2 = {}", n2);

    let b = "true".parse::<bool>().unwrap();

    println!("b = {}", b);

    let addr = "192.169.64.2".parse::<IpAddr>().unwrap();

    println!("addr = {:?}", addr)
}

fn foo1(s: &String) {
    println!("foo(&String) function, s = {s}");
}

fn foo2(s: &str) {
    println!("foo(&str) function, s = {s}");
}

fn bar1(v: &Vec<u8>) {
    println!("bar(&Vec<u8>) function, v = {:?}", v)
}

fn bar2(v: &[u8]) {
    println!("bar(&[u8]) function, v = {:?}", v)
}

fn escape() {
    let s1 = "I'm writing \x52\x75\x73\x74!";
    println!("{}", s1);

    let s2 = "\u{211D}";
    println!("{}", s2);

    let s3 = "select * from customer c join order o \
    on c.id = o.customer_id;";
    println!("{}", s3);

    let s4 = r##"I'm writing \x52\x75\x73\x74! "# "No escape""##;

    println!("{}", s4);

    let s5: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", s5);

    let s6 = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", s6);

    let s7 = br"\u{211D} is not escaped here";
    println!("{:?}", s7);
}