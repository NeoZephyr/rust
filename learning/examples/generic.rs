fn main() {

    println!("3 + 5 = {}", add(3, 5));
    println!("3.14 + 2.78 = {}", add(3.14f64, 2.78f64));

    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 3.14f64, y: 2.78f64};
    println!("point: {:?}", p1);
    println!("distance: {}", p2.distance_from_origin());

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}