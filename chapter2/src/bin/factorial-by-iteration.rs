fn factorial(number: u32) -> u32 {
    let mut product = 1;
    for i in 1..=number {
        product *= i;
    }
    product
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
