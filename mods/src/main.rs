mod garden;

use garden::vegetables::carrot::Carrot;

fn main() {
    // Using the garden module
    let my_garden = garden::Garden::new(100, String::from("Backyard"));
    println!("Garden size: {}", my_garden.size);
    println!("{}", garden::plant_vegetable());
    println!("Location: {}", my_garden.get_location());

    // Using nested modules
    println!("{}", garden::vegetables::grow_vegetables());

    // Using the Carrot struct with 'use'
    let carrot = Carrot::new();
    println!("{}", carrot.describe());

    // Alternative: using full path without 'use'
    let another_carrot = garden::vegetables::carrot::Carrot::new();
    println!("{}", another_carrot.describe());
}
