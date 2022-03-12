fn main() {
    // let name = "World";
    // println!("Hello, {}!", name);

    let name: &str = "Iqbal";
    println!("Nama saya: {}", name);

    // Variabel di Rust memiliki sifat immutable
    // untuk membuat mutable variabel, bisa menggunakan keyword mut
    let mut age = 20;
    age += 1;
    println!("Umur saya: {}", age);
}
