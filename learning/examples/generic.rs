fn main() {
    type_case1();
    type_case2();
    type_case3();

    wrap_case1();
    wrap_case2();
    wrap_case3();

    transformation_case1();
    transformation_case2();
    transformation_case3();

    iteration_case1();
    iteration_case2();
    iteration_case3();

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

// Point<T> 定义类型参数 T
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

// 变体 V1 带了一个 Point 的负载，
// 变体 V2 带了一个 Vec 的负载
enum A<T, U> {
    V1(Point<T>),
    V2(Vec<U>)
}

// Point<T> 使用类型参数 T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 对具化类型做 impl
impl Point<i32> {
    fn move_right(&mut self) -> i32 {
        self.x = self.x + 1;
        self.x
    }
}

fn type_case1() {
    // 推导参数类型
    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 3.14f64, y: 2.78f64};

    // turbofish 语法提供类型参数信息
    let p3 = Point::<u8>{x: 1, y: 30};

    // let p3 = Point{x: 1, y: 3.14};
    println!("p1: {:?}, p2: {:?}, p3: {:?}", p1, p2, p3);

    let mut p4 = Point{x: 5, y: 6};
    let r = &mut p4;
    r.move_right();
    println!("p4: {:?}", p4)
}

fn print<T: std::fmt::Display>(p: Point<T>) {
    println!("Point(x -> {}, y -> {})", p.x, p.y)
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct B<T, U> {
    x: T,
    y: U
}

// 使用 4 个类型参数：X1、Y1、X2、Y2
impl<X1, Y1> B<X1, Y1> {
    fn mix_up<X2, Y2>(self, other: B<X2, Y2>) -> B<X1, Y2> {
        B {
            x: self.x,
            y: other.y
        }
    }
}

fn type_case2() {
    let p = Point{x: 10, y: 20};
    print(p);

    println!("3 + 5 = {}", add(3, 5));
    println!("3.14 + 2.78 = {}", add(3.14f64, 2.78f64));
}

// newtype 模式
// 用新的类型名字替换里面原来那个类型名字
#[derive(Debug)]
struct List<T>(Vec<T>);
// struct List(Vec<u8>);

use std::collections::HashMap;
struct AAA(Vec<u8>);
struct BBB {
    map: HashMap<String, AAA>
}
type CCC = HashMap<String, Vec<BBB>>;

// 洋葱类型重命名
type MyType<T, U> = HashMap<U, Vec<HashMap<String, Vec<HashMap<String, Vec<T>>>>>>;

fn type_case3() {
    let list = List::<i32>(vec![1, 2, 3]);
    println!("list: {:?}", list)
}

fn wrap_case1() {
    let x = Some("value");
    // let x: Option<&str> = None;

    // x 为 None 就会 panic
    let v = x.expect("value can not be none");

    println!("v = {v}");

    let r = std::env::var("env");
    let r: Result<u32, &str> = Ok(100);

    // 如果 Result 为 Err，就会 panic
    let env = r.expect("result error");

    println!("result = {env}")
}

fn wrap_case2() {
    let x = Some("blue eyes");

    // x 为 None 就会 panic
    // unwrap 不带提示信息
    let xv = x.unwrap();

    println!("xv = {xv}");

    let r: Result<u32, &str> = Ok(2);
    // let r = std::env::var("env");

    // 如果 Result 为 Err，就会 panic
    let rv = r.unwrap();

    println!("rv = {rv}")
}

fn wrap_case3() {
    // let x = Some("blue eyes");
    let x: Option<&str> = None;

    // None 情况下，不会 panic，提供默认值
    let xv = x.unwrap_or("green eyes");

    // None 情况下，不会 panic，提供类型对应的默认值
    let xv = x.unwrap_or_default();

    println!("xv = {xv}");

    let r: Result<u32, &str> = Ok(9);
    let r: Result<u32, &str> = Err("not a number");
    let rv = r.unwrap_or(100);
    let rv = r.unwrap_or_default();

    println!("rv = {rv}")
}

fn transformation_case1() {
    // 在 Option 是 None 的情况下，保持 None 不变
    // map() 会消耗原类型，也就是获取所有权
    let s = Some(String::from("Hello, World!"));
    // let s: Option<&str> = None;
    println!("is some: {}, is none: {}", s.is_some(), s.is_none());
    let sl = s.map(|s| s.len());
    println!("sl = {:?}", sl);

    let v = 12;
    let s = Some(&v);

    // 把 Option<&T> 转换成 Option<T>
    let c = s.cloned();
    println!("before cloned: {:?}, after cloned: {:?}", s, c);

    // 把 Option 或 &Option 转换成 Option<&T>
    let s: Option<String> = Some("Hello, world!".to_string());
    let sl: Option<usize> = s.as_ref().map(|s| s.len());
    println!("s = {:?}, sl = {:?}", s, sl);

    let mut s: Option<String> = Some("Hello, world!".to_string());

    // 把 Option 或 &mut Option 转换成 Option<&mut T>
    let ms = s.as_mut();

    match ms {
        Some(r) => *r = String::from("Hello, rust!"),
        None => {}
    }

    println!("s = {:?}", s);

    // 把 Option 的值拿出去，在原地留下一个 None 值
    let mut s: Option<String> = Some("Hello world!".to_string());
    // let mut s: Option<String> = None;
    let st = s.take();
    println!("task operation, s = {:?}, sv = {:?}", s, st);

    // 原地替换新值，同时把原来那个值抛出来
    let mut s: Option<String> = Some("Hello world!".to_string());
    let sr = s.replace("Hello rust!".to_string());
    println!("replace operation, s = {:?}, sr = {:?}", s, sr);

    // 如果 Option 是 None，返回 None
    // 如果 Option 是 Some，就把函数或闭包应用到被包裹的内容上，并返回运算后的结果
    let s = Some("Hello world!".to_string());
    // let s: Option<String> = None;
    let sl = s.as_ref().and_then(|x| Some(x.len()));
    println!("and then operation, s = {:?}, sl = {:?}", s, sl);
}

fn transformation_case2() {
    let s = "123";
    let sr1 = s.parse::<u32>();

    // map：当 Result 是 Ok 的时候，把 Ok 里的类型通过函数运算可以转换成另外一种类型
    // 当 Result 是 Err 的时候，原样返回 Err 和它携带的内容

    // as_ref：从 Result 到 Result<&T, &E>。原来那个 Result 实例保持不变

    let sr2 = sr1.as_ref().map(|r| r * 2);

    println!("map operation, s = {:?}, sr1 = {:?}, sr2 = {:?}", s, sr1, sr2);
    println!("is ok: {}, is err: {}", sr1.is_ok(), sr1.is_err());

    // 当 Result 是 Ok 时，把函数应用到 Ok 携带的内容上面，并返回一个新的 Result
    // 当 Result 是 Err 的时候，这个方法直接传递返回这个 Err 和它的负载
    let sr3 = sr1.as_ref().and_then(|s| Ok(s * 2));
    println!("sr3 = {:?}", sr3);

    // 当 Result 是 Ok 时，传递原样返回
    // 当 Result 是 Err 时，对 Err 携带的内容使用函数进行运算及类型转换
    let sr4: Result<String, String> = Ok("hello world!".to_string());
    let sr4: Result<String, String> = Err("OOM".to_string());
    let error = sr4.as_ref().map_err(|x| format!("error code: {x}"));
    println!("sr4 = {:?}, error = {:?}", sr4, error)
}

fn transformation_case3() {
    let s = Some(100);
    let s: Option<u32> = None;

    // 从 Option<T> 到 Result<T, E>
    let r = s.ok_or("foo");
    println!("s = {:?}, r = {:?}", s, r);

    // 从 Result<T, E> 到 Option<T>
    let r: Result<u32, &str> = Ok(100);
    let r: Result<u32, &str> = Err("bar");
    let s = r.ok();
    println!("s = {:?}, r = {:?}", s, r);

    let r: Result<u32, &str> = Ok(100);
    // let r: Result<u32, &str> = Err("bar");
    let s = r.err();
    println!("s = {:?}, r = {:?}", s, r);
}

fn iteration_case1() {
    let mut v = vec![1, 2, 3, 4, 5];

    // 获取集合元素不可变引用的迭代器
    let mut iter = v.iter();

    // 如果这个集合被迭代完成了，那么最后一次执行会返回 None
    while let Some(i) = iter.next() {
        println!("iter {i}")
    }

    println!("v = {:?}", v);

    // 获取集合元素可变引用的迭代器
    let mut iter = v.iter_mut();

    while let Some(i) = iter.next() {
        println!("iter_mut {i}")
    }

    println!("v = {:?}", v);

    // 获取集合元素所有权的迭代器 into_iter()
    let mut iter = v.into_iter();

    while let Some(i) = iter.next() {
        println!("into_iter {i}")
    }

    // println!("v = {:?}", v)
}

fn iteration_case2() {
    let mut v = vec![1, 2, 3, 4, 5];

    // 相当于调用 c.iter()
    for i in &v {
        println!("for ref {i}")
    }

    println!("v = {:?}", v);

    for i in &mut v {
        println!("for mut ref {i}")
    }

    println!("v = {:?}", v);

    // for 语句默认使用获取元素所有权的迭代器模式
    for i in v {
        println!("for {i}")
    }

    // println!("v = {:?}", v)
}

fn iteration_case3() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let v = vec![s1, s2, s3];

    // 下标索引这种不安全的操作，禁止获得集合元素所有权
    // 迭代器这种安全的操作，允许它获得集合元素所有权
    // let s = v[0];
    let s = &v[0];
    let s = v[0].clone();
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}