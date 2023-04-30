fn factorial(number: u32) -> u32 {
    // The original example uses 1, rather than 0, as the base case, causing
    // factorial(0) to never return.  That bug is fixed here.
    match number {
        0 => 1,                              // BASE CASE
        _ => number * factorial(number - 1), // RECURSIVE CASE
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorial() {
        for (n, want) in [1, 1, 2, 6, 24, 120].into_iter().enumerate() {
            assert_eq!(factorial(n as u32), want);
        }
    }
}

fn main() {
    println!("{}", factorial(5));
}
