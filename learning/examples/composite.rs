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

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Pie;

fn structure() {
    let mut u1 = User {
        active: true,
        name: String::from("jack"),
        email: String::from("jack@qq.com"),
        ttl: 3600
    };
    let u2 = User {
        name: String::from("peter"),
        ..u1
    };
    u1.active = false;

    // value borrowed here after partial move
    // println!("u1: {:?}", u1);
    println!("u2: {:?}", u2);

    let black = Color(0, 0, 0);
    println!("color: {:?}", black);

    let pie = Pie;

    println!("pie: {:?}", pie);
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