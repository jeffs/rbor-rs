fn fibonacci(nth_number: usize) -> u32{
    let (mut a, mut b) = (1, 1);
    println!("a = {a}, b = {b}");
    for _ in 1..nth_number {
        (a, b) = (b, a + b);    // Get the next Fibonacci number.
        println!("a = {a}, b = {b}");
    }
    a
}

fn main() {
    println!("{}", fibonacci(10));
}
