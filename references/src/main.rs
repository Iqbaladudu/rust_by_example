struct Point {
    x: u8,
    y: u8,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    // 2. Add '&' before variable name to peform references
    let p2 = &p1;
    println!("{}", p1.x); // 1. will cause error, because p1 is moved instead of being copied

    // Using references in function parameter
    // This is a function that prints a point, without moving the value
    fn print_point(point: &Point) {
        println!("x: {}, y: {}", point.x, point.y);
    }

    // We can still use the point after calling print_point, because we send a reference to the function instead of moving the point into the function.

    print_point(&p1)
}
