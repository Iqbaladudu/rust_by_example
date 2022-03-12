// Define struct
// #[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Using struct
    let point = Point { x: 24, y: 56 };
    println!("({}, {})", point.x, point.y);
    // println!("{:?}", point);
}
