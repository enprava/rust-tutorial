fn main() {
    let a = "Stack";
    let b = a;
    println!("A: {a}, B: {b}");

    let c = String::from("Heap");
    let d = c;
    // println!("C: {c}"); // Doesn't compile
    println!("D: {d}");
    let e = String::from("Give onweship");
    take_ownership(e);
    // println!("{e}"); // Doesn't compile
    let e = String::from("Give and return onweship");
    let e = take_and_give_ownership(e);
    println!("{e}");
}

fn take_ownership(e: String) {
    println!("{e}");
}

fn take_and_give_ownership(e: String) -> String {
    println!("{e}");
    e
}
