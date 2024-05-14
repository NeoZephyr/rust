use std::collections::HashMap;

fn main() {
    tuple();
    array();
    structure();
    enumeration();
    match_case1();
    match_case2();
    match_case3();
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

fn vector() {
    let vec: Vec<i32> = Vec::new();
    let vec = vec![1, 2, 3];
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    println!("{:?}", vec)
}

fn hash() {
    let mut map = HashMap::new();
    map.insert(String::from("jack"), 30);
    map.insert(String::from("jhon"), 50);
    println!("{:?}", map)
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

#[cfg(test)]
mod tests {
    use crate::Rectangle;

    #[test]
    fn test_rectangle() {
        let r = Rectangle {
            width: 10,
            height: 10,
        };

        assert_ne!(r.area(), 100)
    }
}

#[derive(Debug)]
enum Gender {
    Male,
    Female
}

// 空枚举，不能被实例化
enum Foo {}

// 给枚举变体一个起始数字值
enum Number {
    Zero = 0,
    One,
    Two,
}

#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

// enum 中的变体可以挂载各种形式的类型
#[derive(Debug)]
enum Shape {
    // 把结构体 Rectangle 挂载到 Rectangle 变体上
    Rectangle(Rectangle),

    // 挂载一个元组负载
    Triangle((u32, u32), (u32, u32), (u32, u32)),

    // 挂载了一个结构体负载，表示一个原点加半径长度
    Circle {origin: (u32, u32), radius: u32},
}

impl Shape {
    fn area(&self) -> u32 {
        match self {
            Shape::Rectangle(r) => {
                r.width * r.height
            }
            _ => 0
        }
    }
}

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn enumeration() {
    let man = Gender::Male;
    let woman = Gender::Female;

    println!("man gender is {:?}, woman gender is {:?}", man, woman);

    println!("One is {}", Number::One as u32);

    println!("card1 is {:?}, card2 is {:?}", PokerCard::Clubs(7), PokerCard::Diamonds('Q'));

    let e1 = WebEvent::PageLoad;
    let e2 = WebEvent::KeyPress('A');
    let e3 = WebEvent::Paste(String::from("abc"));
    let e4 = WebEvent::Click {x: 10, y: 10};

    println!("event1 is {:?}, event2 is {:?}, event3 is {:?}, event4 is {:?}", e1, e2, e3, e4);

    println!("rectangle shape is {:?}", Shape::Rectangle(Rectangle{width: 10, height: 20}));

    println!("triangle shape is {:?}", Shape::Triangle((1, 1), (2, 2), (10, 5)));

    println!("circle shape is {:?}", Shape::Circle {origin: (1, 1), radius: 1});
}

fn match_case1() {
    let rect = Shape::Rectangle(Rectangle { width: 10, height: 20 });

    // match 表达式中各个分支返回的值的类型必须相同
    // 所有分支都必须处理
    let ret = match rect {
        Shape::Rectangle(_) => 1,
        Shape::Triangle(_, _, _) => 2,
        Shape::Circle { origin: (x, y), radius: _ } if x == 0 && y == 0  => 3,
        Shape::Circle { origin: _, radius: 0 } => 4,
        Shape::Circle { .. } => 5
    };

    // .. 忽略剩余值
    if let Shape::Triangle(_, ..) = rect {
        println!("is triangle")
    } else {
        println!("is not triangle")
    }

    let mut value = Some(100);

    if let Some(v) = value {
        println!("value is {v}")
    }

    while let Some(v) = value {
        if v > 0 {
            println!("value is {v}");
            value = Some(v / 2);
        } else {
            value = None
        }
    }
}

fn match_case2() {
    let number = 13;

    match number {
        1 => println!("number 1"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("prime number"),
        14..=99 => println!("less than 100"),
        _ => println!("grater number")
    }
}

fn match_case3() {
    let rect = Shape::Rectangle(Rectangle { width: 10, height: 20 });

    // 利用模式匹配解开负载
    let Shape::Rectangle(r) = rect else {
        panic!("can not extract rect")
    };
    println!("rect width: {}, height: {}", r.width, r.height);

    let circle = Shape::Circle {origin: (1, 1), radius: 10};

    // 解开负载，同时定义了 origin 和 radius 两个局部变量
    let Shape::Circle {origin, radius} = circle else {
        panic!("can not extract circle")
    };
    println!("circle origin: ({}, {}), radius: {}", origin.0, origin.1, radius);

    // 解开结构体负载
    let rect = Rectangle {width: 10, height: 20};
    let Rectangle {width, height} = rect;
    println!("rect width: {}, height: {}", width, height);

    // 模式中的变量名不必与结构体中的字段名一致
    let r @ Rectangle {width: w, height: h} = rect;
    println!("rect width: {}, height: {}", w, h);
    println!("rect: {:?}", r);

    let mut user = User {
        name: String::from("jack"),
        email: String::from("abc@qq.com"),
        ttl: 100,
        active: true
    };

    // ttl 和 active 采用了复制所有权的形式
    // name 和 email 字符串值则是采用了移动所有权的形式
    // 通过 ref 这个关键字修饰符告诉 Rust 编译器，现在只需要获得那个字段的引用，不需要所有权
    let User {ref name, ref mut email, ttl, active} = user;
    println!("user name: {}, active: {}, user: {:?}", name, active, user);

    foo(user);

    // 元组的析构，常用来从函数的多个返回值里取出数据
    let tup = ("jack", 165, 60);
    let (name, height, weight) = tup;
    println!("name: {}, height: {}, weight: {}", name, height, weight);

    bar(tup);
}

fn foo(User {name, email, active, ttl}: User) {
    println!("user name: {name}, email: {email}, active: {active}, ttl: {ttl}")
}

fn bar((name, height, weight): (&str, u32, u32)) {
    println!("name: {}, height: {}, weight: {}", name, height, weight)
}

fn closure() {
    // 闭包
    let square = |x: u32| -> u32 {x * x};
    println!("square of 5: {}", square(5));

    // 闭包可以捕获函数中的局部变量
    let factor = 10;

    // fn multiply_v1(x: u32) -> u32 {x * factor}
    let multiply_v2 = |x| {x * factor};
    println!("multiply 12: {}", multiply_v2(12))
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn filter_ipv4() {
    let ips = vec![
        IpAddr::V4(String::from("127.0.0.1")),
        IpAddr::V6(String::from("::1")),
        IpAddr::V4(String::from("192.168.4.4"))
    ];

    let filter_ips: Vec<&IpAddr> = ips.iter().filter(|x| matches!(x, IpAddr::v4(_))).collect();

    println!("filter_ips: {:?}", filter_ips);

    for (index, value) in filter_ips.iter().enumerate() {
        println!("{}, {:?}", index, value)
    }
}
