fn factorial(number: u32) -> u32 {
    let mut product = 1;
    for i in 1..=number {
        product *= i;
    }
    product
}

fn main() {
    println!("{}", factorial(5));
}
