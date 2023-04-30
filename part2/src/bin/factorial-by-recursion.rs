fn factorial(number: u32) -> u32 {
    // The original example uses 1, rather than 0, as the base case, causing a
    // factorial(0) to never return.  That bug is fixed here.
    match number {
        0 => 1,                              // BASE CASE
        _ => number * factorial(number - 1), // RECURSIVE CASE
    }
}

fn main() {
    println!("{}", factorial(5));
}
