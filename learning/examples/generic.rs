fn main() {
    type_case1();
    type_case2();
    type_case3();

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

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}