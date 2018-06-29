fn main() {
    for number in 1..101 {
        match number {
            n if (n % 15 == 0) => println!("CracklePop"),
            n if (n % 3 == 0) => println!("Crackle"),
            n if (n % 5 == 0) => println!("Pop"),
            _ => println!("{}", number),
        }
    }
}
