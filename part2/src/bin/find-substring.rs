fn find_substring_iterative(needle: &str, haystack: &str) -> isize {
    for i in 0..(haystack.len() - needle.len()) {
        if &haystack[i..(i + needle.len())] == needle {
            return i as isize; // Needle found.
        }
    }
    -1 // Needle not found.
}

fn find_substring_recursive(needle: &str, haystack: &str) -> isize {
    fn imp(needle: &str, haystack: &str, i: usize) -> isize {
        if i + needle.len() >= haystack.len() {
            -1 // BASE CASE (Needle not found.)
        } else if &haystack[i..(i + needle.len())] == needle {
            i as isize // BASE CASE (Needle found.)
        } else {
            imp(needle, haystack, i + 1) // RECURSIVE CASE
        }
    }
    imp(needle, haystack, 0)
}

fn main() {
    println!("{}", find_substring_iterative("cat", "My cat Zophie"));
    println!("{}", find_substring_recursive("cat", "My cat Zophie"));
}
