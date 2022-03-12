fn main() {
    /*

       1. Integer

       Unsigned | Signed
       u8 => 0 -> 255 | i8 => -128 -> 127
       u16 => 0 -> 65535 | i16 => -32768 -> 32767
       u32 => 0 -> 4294967295 | i32 => -2147483648 -> 2147483647
       u64 => 0 -> 18446744073709551615 | i64 => -9223372036854775808 -> 9223372036854775807
       u128 => 0 => 340282366920938463463374607431768211455
    | i128 => -170141183460469231731687303715884105728 -> 170141183460469231731687303715884105727

       usize | isize => keduanya merupakan ukuran bit sesuai dengan arsitektur komputer. Secara default komputer 64-bit akan berukuran 64, begitu pun dengan 32-bit

       DEFAULT INTEGER TYPES is i32

       2. Floating Point
           a. f32
           b. f64
           DEFAULT = f64

       3. Boolean
           a. True
           b. False

       4. Charracter Type
           => Merepresentasikan karakter unikode

       */

    let age: u8 = 255;

    println!("Umur: {}", age);
}
