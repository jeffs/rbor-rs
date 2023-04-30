fn main() {
    println!("Code in a loop:");
    let mut i = 0;
    while i < 5 {
        println!("{i} Hello, world!");
        i += 1
    }

    println!("Code in a function:");
    fn hello(mut i: usize) {
        println!("{i} Hello, world!");
        i += 1;
        if i < 5 {
            // RECURSIVE CASE
            hello(i);
        } else {
            // BASE CASE
        }
    }

    hello(0);
}
