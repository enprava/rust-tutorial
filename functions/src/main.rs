fn main() {
    another_function(25);
    let y = plus_one_expression(7);
    println!("Value of y after expression: {y}");

    let z = {
        let z = 100;
        println!("Value of z: {z}");
        z + 1
    };
    println!("Value of z: {z}");
}

fn another_function(x: i32) {
    println!("Value of x: {x}");
    println!("Antoher function!");
}

fn plus_one_expression(y: i32) -> i32 {
    println!("Value of y: {y}");
    y + 1
}
