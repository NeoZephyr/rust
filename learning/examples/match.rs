use crate::IpAddr::{Ipv4, Ipv6};

fn main() {
    play(Action::Move(10, 30));
    play(Action::Move(0, 30));
    play(Action::Move(-10, 30));
    play(Action::Shot);
    play(Action::Pass(String::from("jack")));

    if_match();
    filter_ipv4();
    ignore_match();
}

enum Action {
    Move(i32, i32),
    Shot,
    Pass(String)
}

fn play(action: Action) {
    match action {
        Action::Move(x, 0) => {
            println!("player move to x axis")
        }
        Action::Move(0, y) => {
            println!("player move to y axis")
        }
        Action::Move(x, y) if x < 0 => {
            println!("player move to left region ({}, {})", x, y)
        }
        Action::Move(x, y) => {
            println!("player move to ({}, {})", x, y)
        }
        Action::Shot => {
            println!("player shot ball")
        }
        Action::Pass(_) => {
            println!("player pass to other player")
        }
    }
}

#[derive(Debug)]
enum IpAddr {
    Ipv4(String),
    Ipv6(String)
}

fn filter_ipv4() {
    let ips = vec![Ipv4(String::from("127.0.0.1")), Ipv6(String::from("::1")), Ipv4(String::from("192.168.4.4"))];

    let filter_ips: Vec<&IpAddr> = ips.iter().filter(|x| matches!(x, IpAddr::Ipv4(_))).collect();

    println!("filter_ips: {:?}", filter_ips);

    for (index, value) in filter_ips.iter().enumerate() {
        println!("{}, {:?}", index, value)
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn ignore_match() {
    let point = Point{x: 100, y: 200, z: 300};

    match point {
        // .. 忽略剩余值
        Point {x, ..} => println!("point x axis location: {}", x)
    }
}

fn if_match () {
    let v = Some(1000);
    match v {
        Some(i) => println!("value: {}", i),
        _ => (),
    }

    // 只要匹配一个条件
    if let Some(i) = v {
        println!("value: {}", i)
    }

    let p1 = Point { x: 20, y: 60, z: 100 };

    let Point { x, y, z} = p1;
    println!("point location: ({}, {}, {})", x, y, z);

    // 模式中的变量名不必与结构体中的字段名一致
    let p2 @ Point {x: a, y: b, z} = p1;
    println!("point location: ({}, {}, {})", a, b, z);
    println!("point p2: {:?}", p2)
}