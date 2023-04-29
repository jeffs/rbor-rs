/// Factorial in log(N) time without recursion.

// Exercises for the reader:
//
// * Use a stack-based bitvector rather than a Vec.
// * Return a BigInt rather than a fixed size type.
//   - Hint: see [num_bigint](https://docs.rs/num-bigint/latest/num_bigint/)
pub fn pow(a: i32, mut n: u32) -> i32 {
    let mut carries = vec![];
    while n != 0 {
        carries.push(n % 2);
        n /= 2;
    }
    let mut result = 1;
    while let Some(carry) = carries.pop() {
        result *= result;
        if carry != 0 {
            result *= a;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pow() {
        for a in 0..5 {
            assert_eq!(pow(a, 0), 1);
            assert_eq!(pow(a, 1), a);
            assert_eq!(pow(a, 2), a * a);
            assert_eq!(pow(a, 3), a * a * a);
            assert_eq!(pow(a, 4), a * a * a * a);
            assert_eq!(pow(a, 5), a * a * a * a * a);
            assert_eq!(pow(a, 6), a * a * a * a * a * a);
        }
    }
}
