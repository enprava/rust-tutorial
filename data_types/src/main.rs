fn main() {
    let sum = 5 + 10;
    println!("Sum : {sum}");
    let difference = 95.5 - 4.3;
    println!("Diff: {difference}");
    let product = 4 * 30;
    println!("Product: {product}");
    let quotient = 56.7 / 32.2;
    println!("Quotient: {quotient}");
    let truncated = -5 / 3;
    println!("Truncated: {truncated}");
    let remainder = 43 % 5;
    println!("Remainder: {remainder}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    println!("500: {five_hundred}");
    let six_point_four = x.1;
    println!("6.4: {six_point_four}");
    let one = x.2;
    println!("1: {one}");
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let january = months[0];
    println!("January: {january}");
}
