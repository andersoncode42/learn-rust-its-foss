fn main() {
    // one character array
    // WITHOUT type annotation
    let greeting = ['H', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!'];

    // the first 10 values of Pi after the decimal values stored in it as individual numbers
    // WITH type annotation
    let pi: [i32; 10] = [1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    for character in greeting {
        print!("{}", character);
    }

    println!("\nPi: 3.1{}{}{}{}", pi[0], pi[1], pi[2], pi[3]);
}