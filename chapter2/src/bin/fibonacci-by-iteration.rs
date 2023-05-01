fn fibonacci(nth_number: usize) -> u32 {
    let (mut a, mut b) = (0, 1);
    println!("a = {a}, b = {b}");
    for _ in 0..nth_number {
        (a, b) = (b, a + b); // Get the next Fibonacci number.
        println!("a = {a}, b = {b}");
    }
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci() {
        for (n, want) in [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
            .into_iter()
            .enumerate()
        {
            let got = fibonacci(n);
            eprintln!("n = {n}, got = {got}");
            assert_eq!(got, want);
        }
    }
}

fn main() {
    println!("{}", fibonacci(10));
}
