#[derive(Clone, Debug)]

struct Point {
    x: i32,
    y: i32,
}

fn print_point(point: Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    print_point(p1.clone());
    println!("{}", p2.x)
}
