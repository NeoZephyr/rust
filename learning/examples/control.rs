fn main() {
    looop()
}

fn condition() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let score = 80;

    // if else 支持表达式返回
    let stage = if score >= 60 {
        "pass"
    } else {
        "fail"
    };
    println!("stage: {}", stage)
}

fn looop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // 跳出循环，同时带一个返回值回去
            break counter * 2
        }
    };
    println!("result: {}", result);

    while counter < 50 {
        counter *= 2;
    }

    println!("counter: {}", counter);

    let scores = [64, 78, 99];

    for elem in scores {
        println!("score value: {}", elem)
    }

    // 左闭右闭区间
    for c in (1..=10).rev() {
        println!("{c}")
    }

    for ch in 'a'..'z' {
        println!("{ch}")
    }
}