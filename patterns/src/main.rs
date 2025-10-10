fn main() {
    println!("=== Pattern Matching Examples ===\n");

    // 1. Basic match expression
    println!("1. Basic Match Expression:");
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    // 2. Matching with multiple patterns
    println!("\n2. Multiple Patterns:");
    let x = 5;
    match x {
        1 | 2 | 3 => println!("Small number"),
        4 | 5 | 6 => println!("Medium number"),
        _ => println!("Large number"),
    }

    // 3. Matching ranges
    println!("\n3. Range Matching:");
    let age = 25;
    match age {
        0..=17 => println!("Minor"),
        18..=64 => println!("Adult"),
        65..=120 => println!("Senior"),
        _ => println!("Invalid age"),
    }

    // 4. Destructuring structs
    println!("\n4. Struct Destructuring:");
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    match point {
        Point { x: 0, y: 0 } => println!("At origin"),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y } => println!("At ({}, {})", x, y),
    }

    // 5. Destructuring enums
    println!("\n5. Enum Destructuring:");
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(255, 0, 0);
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }

    // 6. Matching with guards
    println!("\n6. Pattern Guards:");
    let num = Some(15);
    match num {
        Some(x) if x < 10 => println!("Less than 10: {}", x),
        Some(x) if x % 2 == 0 => println!("Even number: {}", x),
        Some(x) => println!("Odd number greater than 10: {}", x),
        None => println!("No number"),
    }

    // 7. @ bindings
    println!("\n7. @ Bindings:");
    let value = 42;
    match value {
        x @ 0..=50 => println!("Value {} is in range 0-50", x),
        x @ 51..=100 => println!("Value {} is in range 51-100", x),
        x => println!("Value {} is out of range", x),
    }

    // 8. if let syntax
    println!("\n8. if let Syntax:");
    let some_value = Some(7);

    // Instead of match for single case
    if let Some(x) = some_value {
        println!("Got a value: {}", x);
    }

    // if let with else
    let none_value: Option<i32> = None;
    if let Some(x) = none_value {
        println!("Got a value: {}", x);
    } else {
        println!("Got nothing");
    }

    // 9. while let syntax
    println!("\n9. while let Syntax:");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    print!("Popping from stack: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // 10. Pattern matching in function parameters
    println!("\n10. Function Parameter Patterns:");

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let coords = (3, 7);
    print_coordinates(&coords);

    // 11. Matching slices
    println!("\n11. Slice Patterns:");
    let arr = [1, 2, 3];
    match arr {
        [1, _, _] => println!("Starts with 1"),
        [a, b, c] => println!("Array: [{}, {}, {}]", a, b, c),
    }

    // 12. Ignoring values in patterns
    println!("\n12. Ignoring Values:");
    let triple = (1, 2, 3);
    match triple {
        (first, _, third) => println!("First: {}, Third: {}", first, third),
    }

    // 13. Matching with .. to ignore remaining parts
    println!("\n13. Ignoring Remaining Parts:");
    struct ComplexStruct {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let complex = ComplexStruct {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };
    match complex {
        ComplexStruct { a, b, .. } => println!("a: {}, b: {}", a, b),
    }

    println!("\n=== Pattern Matching Complete ===");
}

