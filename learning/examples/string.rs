fn main() {
    slice();
    ownership();
    escape();
    utf8();
}

fn slice() {
    let mut s = String::from("hello");
    s.push_str(", rust");
    s.push('!');

    println!("{}", s);

    let r1 = &s;
    let r2 = &s[7..];
    let r3 = s.as_str();

    //  mutable borrow occurs here
    // let r4 = &mut s;

    println!("r1: {}", r1);
    println!("r2: {}", r2);
    println!("r3: {}", r3);

    // first mutable borrow occurs here
    let r4 = &mut s;

    // second mutable borrow occurs here
    // s.clear();

    println!("r4: {}", r4);
}

fn ownership() {
    let s1 = String::from("hello, ");
    let s2 = String::from("rust");

    // 深拷贝
    let mut s3 = s1.clone() + &s2;
    s3 = s3 + "!";
    println!("{}", s3);

    let s4 = format!("{}{}", s1, s2);
    println!("{}", s4);

    // 一个值只有一个所有者（避免二次释放），所有权被转移（浅拷贝）之后，原所有者就失效了
    // value moved here
    s3 = s1 + &s2;
    s3 = s3 + "!";
    println!("{}", s3);

    // 获取变量的引用，称之为借用（borrowing）
    // value borrowed here after move
    // let s4 = format!("{}{}", s1, s2);
    // println!("{}", s4)

    // s5 只是引用了存储在二进制中的字符串，并没有持有所有权
    let s5 = "hello, rust!";

    // 仅仅是对引用进行了拷贝
    let s6 = s5;
    println!("{}, {}", s5, s6);

    // let s7 = String::from("hello, rust!");
    // let s8 = s7;
    // println!("{}, {}", s7, s8);

    // 同一作用域，特定数据只能有一个可变引用
    // 可变引用与不可变引用不能同时存在
    // let mut s9 = String::from("hello, rust!");
    // let r1 = &mut s9;
    // let r2 = &s9;
    // let r3 = &mut s9;
    //
    // println!("{}, {}, {}", r1, r2, r3);
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
}

fn utf8() {
    let s = "牛顿万有引力";

    println!("char stream");

    for c in s.chars() {
        println!("{}", c)
    }

    println!("byte stream");

    for b in s.bytes() {
        println!("{}", b)
    }
}