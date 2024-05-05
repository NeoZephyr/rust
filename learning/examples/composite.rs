use std::collections::HashMap;

fn main() {
    tuple();
    array();
    structure();
    enumeration();
}

fn tuple() {
    // 与数组的相同点是，元素个数固定
    // 与数组的不同点是，元素的类型可以不一样
    let tup = ("jack", 165, 60);
    let (name, height, _) = tup;
    println!("name: {}, height: {}cm, weight: {}kg", name, height, tup.2)
}

fn array() {
    let colors = ["blue", "red", "yellow", "pink", "white"];
    let scores = [3.14; 5];
    let words: [String; 5] = std::array::from_fn(|_i| String::from("hello, Rust!"));

    // 固定长度
    let months = [
        "January", "February", "March",
        "April", "May", "June",
        "July", "August", "September",
        "October", "November", "December"];
    println!("{:?}", months);

    println!("{:?} {:?} {:?} {:?}", colors, scores, words, &colors[1..4])
}

#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    email: String,
    ttl: u64,
}

#[derive(Debug, Default)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area1(self: Self) -> u32 {
        self.width * self.height
    }

    fn area2(self: &Self) -> u32 {
        self.width * self.height
    }

    fn area3(self: &mut Self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数（静态方法）
    fn numbers(rows: u32, cols: u32) -> u32 {
        rows * cols
    }
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height
        }
    }
}

// 匿名结构体，也叫做元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);

// 单元结构体
#[derive(Debug)]
struct Pie;

fn structure() {
    let name = String::from("jack");
    let mut u1 = User {
        active: true,
        name,
        email: String::from("jack@qq.com"),
        ttl: 3600
    };
    let u2 = User {
        name: String::from("peter"),
        ..u1
    };
    u1.active = false;

    // 部分移动
    // println!("u1: {:?}", u1);
    println!("u2: {:?}", u2);

    let black = Color(0, 0, 0);
    println!("color: {:?}", black);

    let pie = Pie;

    println!("pie: {:?}", pie);

    let rect1 = Rectangle {
        width: 3,
        height: 4,
    };
    let rect2 = Rectangle::new(5, 9);
    let rect3 = Rectangle::default();
    let rect4: Rectangle = Default::default();
    let r1 = &rect1;
    let r2 = &&rect1;

    // println!("area of rectangle: {}", rect1.area1());
    println!("area of rectangle: {}", r1.area2());
    println!("area of rectangle: {}", r2.area2());

    println!("rect2: {:?}, rect3: {:?}, rect4: {:?}", rect2, rect3, rect4);

    Rectangle::numbers(10, 10);
}

fn vector() {
    let vec: Vec<i32> = Vec::new();
    let vec = vec![1, 2, 3];
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    println!("{:?}", vec)
}

fn hash() {
    let mut  map = HashMap::new();
    map.insert(String::from("jack"), 30);
    map.insert(String::from("jhon"), 50);
    println!("{:?}", map)
}

enum Gender {
    Male,
    Female
}

enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

fn enumeration() {
    let man = Gender::Male;
    let woman = Gender::Female;
    let c1 = PokerCard::Clubs(5);
    let c2 = PokerCard::Diamonds('K');

    let x1 = Some(5);
    let x2: Option<i32> = None;
    let x3 = incr(x1);
    let x4 = incr(x2);
}

fn incr(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}