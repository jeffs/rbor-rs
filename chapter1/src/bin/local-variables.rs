fn a() {
    let spam = "Ant";
    println!("spam is {spam}");
    b();
    println!("spam is {spam}");
}

fn b() {
    let spam = "Bobcat";
    println!("spam is {spam}");
    c();
    println!("spam is {spam}");
}

fn c() {
    let spam = "Coyote";
    println!("spam is {spam}");
}

fn main() {
    a();
}
