fn main() {
    fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }

    println!("{}", max(2, 3))
}
