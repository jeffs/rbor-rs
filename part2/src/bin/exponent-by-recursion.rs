fn exponent_by_recursion(a: i64, n: usize) -> i64 {
    if n == 1 {
        // BASE CASE
        a
    } else if n % 2 == 0 {
        // RECURSIVE CASE (When n is even.)
        let result = exponent_by_recursion(a, n / 2);
        result * result
    } else {
        // RECURSIVE CASE (When n is odd.)
        let result = exponent_by_recursion(a, n / 2);
        result * result * a
    }
}

fn main() {
    println!("{}", exponent_by_recursion(3, 6));
    println!("{}", exponent_by_recursion(10, 3));
    println!("{}", exponent_by_recursion(17, 10));
}
