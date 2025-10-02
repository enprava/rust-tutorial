fn main() {
    let reference = no_dangle();
    println!("{reference}");
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}
// fn dangle()-> &String{
//     let s = String::from("hello");
//                                      Doesn't compile
//     &s
// }
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
