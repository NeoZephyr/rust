fn main() {
    let circle = Circle::new(10f64, 10f64, 5f64);
    println!("circle area: {}", circle.area());

    // 闭包
    let square = |x: u32| -> u32 {x * x};
    println!("square of 5: {}", square(5));

    // 闭包可以捕获函数中的局部变量
    let factor = 10;
    // fn multiply_v1(x: u32) -> u32 {x * factor}
    let multiply_v2 = |x| {x * factor};
    println!("multiply 12: {}", multiply_v2(12))
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

#[cfg(test)]
mod tests {
    use crate::Circle;

    #[test]
    fn test_circle() {
        let c = Circle {
            x: 10.0,
            y: 10.0,
            radius: 4.0
        };
        assert_ne!(c.area(), 10.0)
    }
}