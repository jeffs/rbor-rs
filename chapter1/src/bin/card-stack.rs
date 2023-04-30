#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mut card_stack = Vec::new();

        card_stack.push("5 of diamonds");
        assert_eq!(card_stack, ["5 of diamonds"]);

        card_stack.push("3 of clubs");
        assert_eq!(card_stack, ["5 of diamonds", "3 of clubs"]);

        card_stack.push("ace of hearts");
        assert_eq!(card_stack, ["5 of diamonds", "3 of clubs", "ace of hearts"]);

        card_stack.pop();
        assert_eq!(card_stack, ["5 of diamonds", "3 of clubs"]);
    }
}

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
