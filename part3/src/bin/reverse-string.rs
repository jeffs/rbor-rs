fn rev(the_string: &str) -> String {
    let mut chars = the_string.chars();
    let Some(head) = chars.next() else {
        // BASE CASE
        return String::new();
    };

    // RECURSIVE CASE
    let tail: String = chars.collect();
    format!("{}{head}", rev(&tail))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rev() {
        assert_eq!(rev("abcdef"), "fedcba");
        assert_eq!(rev("Hello, world!"), "!dlrow ,olleH");
        assert_eq!(rev(""), "");
        assert_eq!(rev("X"), "X");
    }
}

fn main() {
    println!("{}", rev("abcdef"));
    println!("{}", rev("Hello, world!"));
    println!("{}", rev(""));
    println!("{}", rev("X"));
}
