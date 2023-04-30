fn shortest_with_base_case(make_recursive_call: bool) {
    println!("shortest_with_base_case({make_recursive_call}) called.");
    if !make_recursive_call {
        // BASE CASE
        println!("Returning from base case.");
        return;
    }
    // RECURSIVE CASE
    shortest_with_base_case(false);
    println!("Returning from recursive case.");
}

fn main() {
    println!("Calling shortest_with_base_case(false):");
    shortest_with_base_case(false);
    println!();
    println!("Calling shortest_with_base_case(true):");
    shortest_with_base_case(true);
}
