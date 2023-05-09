fn main() {
    // Elemento: o número 10
    // Qtd de cópias: 5 cópias
    let a = [10; 5]; // [10, 10, 10 ,10 ,10]

    for i in a {
        print!("{i} ");
    }
    println!("");
}