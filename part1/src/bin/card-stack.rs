fn main() {
    let mut card_stack = Vec::new();

    card_stack.push("5 of diamonds");
    println!("{}", card_stack.join(","));

    card_stack.push("3 of clubs");
    println!("{}", card_stack.join(","));

    card_stack.push("ace of hearts");
    println!("{}", card_stack.join(","));

    card_stack.pop();
    println!("{}", card_stack.join(","));
}
