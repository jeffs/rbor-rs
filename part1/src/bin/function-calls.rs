fn a() {
    println!("a was called.");
    b();
    println!("a is returning.");
}

fn b() {
    println!("b was called.");
    c();
    println!("b is returning.");
}

fn c() {
    println!("c was called.");
    println!("c is returning.");
}

fn main() {
    a();
}
