fn main() {
    let number1: u8 = 54;
    let number2: u8 = 42;

    if number1 > number2 {
        println!("{} > {}", number1, number2);
    } else {
        println!("{} <= {}", number1, number2);
    };

    let minimum = if number1 < number2 { number1 } else { number2 };
    println!("Minimum: {}", minimum);

    // While Lopp
    println!("While Loop");

    let mut a: u8 = 15;
    let mut b: u8 = 40;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    println!("Greatest common divisor of 15 and 40 is: {}", a);
}
