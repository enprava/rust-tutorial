fn main() {
    ifs();
    statement_if(true);
    loop_loop();
    while_loop();
    for_loop();
    let farenheit = 70.;
    let celsius = farenheit_to_celsius(farenheit);
    println!("{farenheit} º F are {celsius} º C");
    let fib_n = 20;
    let fib = fibonacci(fib_n);
    println!("Fibonacci {fib_n} is {fib}");
}

fn ifs() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn statement_if(condition: bool) {
    let number = if condition { 5 } else { 6 };
    println!("Number is: {number}");
}

fn loop_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn while_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn farenheit_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32.) * 5. / 9.
}

fn fibonacci(i: u128) -> u128 {
    if i == 1 || i == 2 {
        1
    } else {
        fibonacci(i - 1) + fibonacci(i - 2)
    }
}
