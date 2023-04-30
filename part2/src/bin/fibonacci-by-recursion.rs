fn fibonacci(nth_number: usize) -> u32 {
    println!("fibonacci({nth_number}) called.");
    match nth_number {
        1 | 2 => {
            // BASE CASE
            println!("Call to fibonacci({nth_number}) returning 1.");
            1
        }
        _ => {
            // RECURSIVE CASE
            println!("Calling fibonacci({}) and fibonacci({})", nth_number - 1, nth_number - 2);
            let result = fibonacci(nth_number - 1) + fibonacci(nth_number - 2);
            println!("Call to fibonacci({nth_number}) returning {result}.");
            result
        }
    }
}

fn main() {
    println!("{}", fibonacci(10));
}
