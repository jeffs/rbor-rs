enum Op {
    Multiply,
    Square,
}

use Op::*;

fn exponent_with_power_rule(a: i64, mut n: usize) -> i64 {
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

fn main() {
    println!("{}", exponent_with_power_rule(3, 6));
    println!("{}", exponent_with_power_rule(10, 3));
    println!("{}", exponent_with_power_rule(17, 10));
}
