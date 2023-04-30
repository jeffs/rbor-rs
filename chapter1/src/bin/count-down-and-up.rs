fn count_down_and_up(number: i32) {
    println!("{number}");
    if number == 0 {
        // BASE CASE
        println!("Reached the base case.");
        return;
    }
    // RECURSIVE CASE
    count_down_and_up(number - 1);
    println!("{number} returning");
}

fn main() {
    count_down_and_up(3);
}
