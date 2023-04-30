fn exponent_by_iteration(a: i64, n: usize) -> i64 {
    let mut result = 1;
    for _ in 0..n {
        result *= a;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exponent_by_iteration() {
        assert_eq!(exponent_by_iteration(3, 6), 729);
        assert_eq!(exponent_by_iteration(10, 3), 1000);
        assert_eq!(exponent_by_iteration(17, 10), 2015993900449);
    }
}

fn main() {
    println!("{}", exponent_by_iteration(3, 6));
    println!("{}", exponent_by_iteration(10, 3));
    println!("{}", exponent_by_iteration(17, 10));
}
