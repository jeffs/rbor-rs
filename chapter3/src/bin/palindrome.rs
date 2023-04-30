fn is_palindrome(the_string: &str) -> bool {
    // BASE CASE
    the_string.len() < 2 || {
        // RECURSIVE CASE
        let mut chars = the_string.chars();
        let head = chars.next();
        let last = chars.next_back();
        let middle = chars.as_str();
        head == last && is_palindrome(middle)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        for (word, want) in [
            ("racecar", true),
            ("amanaplanacanalpanama", true),
            ("tacocat", true),
            ("zophie", false),
        ] {
            assert_eq!(is_palindrome(word), want);
        }
    }
}

fn main() {
    let text = "racecar";
    println!("{text} is a palindrome: {}", is_palindrome(text));

    let text = "amanaplanacanalpanama";
    println!("{text} is a palindrome: {}", is_palindrome(text));

    let text = "tacocat";
    println!("{text} is a palindrome: {}", is_palindrome(text));

    let text = "zophie";
    println!("{text} is a palindrome: {}", is_palindrome(text));
}
