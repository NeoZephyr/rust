fn main() {
    // mutable()
    // ownership()
    reference_case1();
    reference_case2();
    reference_case3();
    reference_case4();
    reference_case5();
    reference_case6();
}

fn mutable() {
    let x = 5;
    println!("x = {}", x);

    // 默认变量是不可变的
    // x = 6;

    // 变量的 Shadowing，原来那个变量被遮盖起来，访问不到了
    let x = 'X';
    println!("x = {}", x);

    let mut x = 5;
    println!("x = {}", x);

    x = 6;
    println!("x = {}", x);
}

fn ownership() {
    let n1: u32 = 10;
    let n2 = n1;

    println!("n1 = {}, n2 = {}", n1, n2);

    let s1 = String::from("Hello world");

    // u32 是固定尺寸类型，String 是非固定尺寸类型
    // 固定尺寸类型，默认放在栈上
    // 而非固定尺寸类型，默认创建在堆上，成为堆上的一个资源，然后在栈上用一个局部变量来指向它

    // 字符串的引用由 s1 拷贝到了 s2，保留最新的 s2 到字符串的指向，把 s1 到字符串的指向给抹去了
    // s2 持有那个字符串的所有权
    // s1 处于无效状态
    let s2 = s1;

    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // 1. Rust 中，每一个值都有一个所有者
    // 2. 任何一个时刻，一个值只有一个所有者
    // 3. 当所有者所在作用域结束的时候，其管理的值会被一起释放掉

    // 堆内存资源随着关联的栈上局部变量一起被回收的内存管理特性，叫作 RAII

    // s2 的所有权移动给了函数的参数 s，这个参数是一个局部变量，在函数栈帧结束的时候会被回收，字符串也会被回收
    foo(s2);
    // foo(s2);
    // println!("s2 = {}", s2);

    let s3 = String::from("Hello world");

    // 把所有权转移出来
    let s3 = bar(s3);
    println!("s3 = {}", s3)
}

fn foo(s: String) {
    println!("foo function, s = {}", s)
}

fn bar(s: String) -> String {
    println!("foo function, s = {}", s);
    s
}

fn reference_case1() {
    // 引用也是一种值，并且是固定尺寸的值
    // 1. 可以赋给另一个变量，赋值的时候，采用复制操作
    // 2. 可以引用一个引用
    let a = 10u32;
    let b = &a;
    let c = &&&a;
    let d = &b;

    // 引用的复制操作
    let e = b;

    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
}

fn reference_case2() {
    let u = 10u32;

    // u 是不可变的
    // let y = &mut u;

    let mut x = 10u32;
    let y = &mut x;
    *y = 20;

    // println! 中默认会对所有权变量做不可变借用操作
    // 先打印 x 会报错
    // println!("x = {}", x);
    // x = x + 10;
    println!("y = {}", y);
    println!("x = {}", x);

    let mut m = 10u32;

    // n, k 交换顺序会报错
    let n = &mut m;
    *n = 20;
    let k = &m;

    // 如果 n, k 交换顺序后，换成打印 n 不会报错
    println!("k = {}", k)
}

fn reference_case3() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &mut a;

    // 换成打印 b 就会出错
    println!("c = {c}");
}

fn reference_case4() {
    let mut a = 10u32;
    let b = &a;
    a = 20u32;

    // 提示在有借用的情况下，不能对所有权变量进行更改值的操作
    // println!("b = {b}");
}

fn reference_case5() {
    let mut a = 10u32;
    let b = &mut a;

    // 可变引用的再赋值，会执行移动操作。赋值后，原来的那个可变引用变量就不能用了
    let c = b;

    // println!("b = {}", b)
}

fn reference_case6() {
    let mut s = String::from("Hello world.");
    foo_ref(&mut s);
    println!("s = {s}")
}

fn foo_ref(s: &mut String) {
    println!("foo ref function, s = {s}");
    s.push_str(" Rust!")
}