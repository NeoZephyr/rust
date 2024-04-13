use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // ownership();
    // types_impl_copy_trait();
    // types_not_impl_copy_trait();

    multi_owner();
    dag();

    // string
    // let arr = Arc::new(vec![1]);
    // std::thread::spawn(move || {
    //     println!("child thread, {:?}", *arr);
    // });
    // println!("main thread, {:?}", *arr);
    // sleep(Duration::from_secs(1))
}

/// 1. 一个值同一时刻只能有一个所有者
/// 2. 当所有者离开作用域，其拥有的值被丢弃
/// 3. 赋值或者传参会导致值 Move，所有权被转移，一旦所有权转移，之前的变量就不能访问
///
/// 4. 如果值实现了 Copy trait，那么赋值或传参会使用 Copy 语义，相应的值会被按位拷贝，产生新的值
///
/// 5. 一个值可以有多个只读引用
/// 6. 一个值可以有唯一一个活跃的可变引用。可变引用（写）和只读引用（读）是互斥的关系
/// 7. 引用的生命周期不能超出值的生命周期
///
fn ownership() {
    // 动态数组因为大小在编译期无法确定，所以放在堆上，并且在栈上有一个包含了长度和容量的胖指针指向堆上的内存
    let data = vec![10, 50, 90, 23];
    let v = 90;
    if let Some(pos) = find_pos(data, v) {
        println!("found {} at {}", v, pos)
    }
    // rustc --explain E0382
    // println!("{:?}", data)

    let data = vec![10, 50, 90, 23];

    // 调用 data.clone() 把 data 复制一份出来
    // 这样，在堆上就有 vec![10, 50, 90, 23] 两个互不影响且可以独立释放的副本
    // 但是，这也会让代码变复杂，效率也不高
    let sum = sum(data.clone());
    println!("sum: {}", sum);
    println!("{:?}", data);

    // 当要移动一个值，如果值的类型实现了 Copy trait，就会自动使用 Copy 语义（不希望值的所有权被转移）
    // 这样，在赋值或者传参时，值会自动按位拷贝（浅拷贝）。否则使用 Move 语义进行移动


    // Borrow 语义通过引用语法（& 或者 &mut）来实现
    // Rust 的引用实现了 Copy trait，按照 Copy 语义，引用会被复制一份交给要调用的函数
    // 对这个函数来说，它并不拥有数据本身，数据只是临时借给它使用，所有权还在原来的拥有者那里
    let data = vec![1, 2, 3, 4];
    let ref_data = &data;
    println!("factorial: {}", factorial(ref_data));

    // 对值的引用也要有约束：借用不能超过（outlive）值的生存期
    // let r = local_ref();
    // println!("r: {:p}, {}", r, r);

    // 在堆内存中，使用栈内存的引用
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("data: {:?}", data);

    // 在一个作用域内，仅允许一个活跃的可变引用
    // 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在

    // let mut arr = vec![1, 2, 3];
    // let last = arr.last();
    // arr.push(4);
    // println!("last: {:?}", last);
}

fn multi_owner() {
    let a = Rc::new(10);
    let b = a.clone();
    let c = a.clone();

    println!("a + 1 = {}", *a + 1);
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("strong count, a: {}, b: {}, c: {}", Rc::strong_count(&a), Rc::strong_count(&b), Rc::strong_count(&c));

    let data = RefCell::new(100);

    {
        // 获得 RefCell 内部数据的可变借用
        let mut v = data.borrow_mut();
        *v += 1;
    }

    println!("data: {:?}", data.borrow());
}

// #[allow(dead_code)]
#[derive(Debug)]
struct Node {
    id: usize,
    // 使用 Rc<RefCell<T>> 让节点可以被修改
    downstream: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&mut self) -> Option<Rc<RefCell<Node>>> {
        // self.downstream.as_ref().clone();
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn dag() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap());

    println!("node1: {:?}", node1);
    println!("node2: {:?}", node2);

    // RefRc 允许在运行时，对某个只读数据进行可变借用
    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));

    println!("node1: {:?}", node1);
    println!("node2: {:?}", node2);
}

// fn local_ref<'a>() -> &'a i32 {
//     let a = 3;
//     &a
// }

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos)
        }
    }

    None
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

fn factorial(data: &Vec<u32>) -> u32 {
    data.iter().fold(1, |acc, x| acc * x)
}

fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // 所有整数类型都是 copy
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    // 函数指针是 copy
    is_copy::<fn()>();

    // 裸指针是 copy
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // 不可变引用是 copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    is_copy::<&[u8]>();

    // 对于数组/元组，如果其内部类型是 copy 那么它们也是 copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    // DST 类型不是 copy
    // 非固定大小的数据结构，没有实现 Copy
    // is_copy::<str>();
    // is_copy::<[u8]>();

    // 有堆内存的类型不是 copy
    // is_copy::<Vec<u8>>();
    // is_copy::<String>();

    // 可变引用不是 copy
    // is_copy::<&mut String>();

    // 对于数组/元组/，如果其内部类型是不是 copy，那么它们也不是 copy
    // is_copy::<[Vec<u8>; 4]>();
    // is_copy::<(String, u32)>();
}