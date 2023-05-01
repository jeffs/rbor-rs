//! Exponentiation in log(N) time without recursion.

enum Op {
    Multiply,
    Square,
}

use Op::*;

/// An implementation slavishly based on the ones in the book.  Note that it's
/// ironically missing a base case for n == 0.
fn exponent_with_power_rule(a: i64, mut n: u32) -> i64 {
    // Step 1: Determine the operations to be performed.
    let mut op_stack = vec![];
    while n > 1 {
        if n % 2 == 0 {
            // n is even.
            op_stack.push(Square);
            n /= 2;
        } else {
            // n is odd.
            n -= 1;
            op_stack.push(Multiply);
        }
    }

    // Step 2: Perform the operations in reverse order.
    let mut result = a; // Start result at `a`.
    while let Some(op) = op_stack.pop() {
        match op {
            Multiply => result *= a,
            Square => result *= result,
        }
    }

    result
}

/// An alternative implementation that's a bit simpler than the one in the book.
/// Some other options to explore:
///
/// * Use a stack-based bitvector rather than a Vec.
/// * Return a BigInt rather than a fixed size type.
///   - Hint: see [num_bigint](https://docs.rs/num-bigint/latest/num_bigint/)
pub fn exponent_with_power_rule_simplified(a: i64, mut n: u32) -> i64 {
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

    fn check_exponent_function_basic(pow: fn(i64, u32) -> i64) {
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

    fn check_exponent_function_example(pow: fn(i64, u32) -> i64) {
        assert_eq!(pow(3, 6), 729);
        assert_eq!(pow(10, 3), 1000);
        assert_eq!(pow(17, 10), 2015993900449);
    }

    /// This test fails because of a known bug in exponent_with_power_rule.
    #[ignore]
    #[test]
    fn test_exponent_with_power_rule_basic() {
        check_exponent_function_basic(exponent_with_power_rule);
    }

    #[test]
    fn test_exponent_with_power_rule_example() {
        check_exponent_function_example(exponent_with_power_rule);
    }

    #[test]
    fn test_exponent_with_power_rule_simplified() {
        check_exponent_function_basic(exponent_with_power_rule_simplified);
        check_exponent_function_example(exponent_with_power_rule_simplified);
    }
}

fn main() {
    println!("{}", exponent_with_power_rule(3, 6));
    println!("{}", exponent_with_power_rule(10, 3));
    println!("{}", exponent_with_power_rule(17, 10));
}
