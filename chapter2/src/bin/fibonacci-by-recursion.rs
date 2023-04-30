fn fibonacci(n: usize) -> u32 {
    println!("fibonacci({n}) called.");
    if n < 2 {
        // BASE CASE
        println!("Call to fibonacci({n}) returning 1.");
        return n as u32;
    }
    // RECURSIVE CASE
    println!("Calling fibonacci({}) and fibonacci({})", n - 1, n - 2);
    let result = fibonacci(n - 1) + fibonacci(n - 2);
    println!("Call to fibonacci({n}) returning {result}.");
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci() {
        for (n, want) in [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into_iter().enumerate() {
            let got = fibonacci(n);
            eprintln!("n = {n}, got = {got}");
            assert_eq!(got, want);
        }
    }
}

fn main() {
    println!("{}", fibonacci(10));
}
