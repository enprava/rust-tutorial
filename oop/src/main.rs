// main.rs

// 1. ENCAPSULATION - Using pub to control access
pub struct BankAccount {
    balance: f64,           // private field
    account_number: String, // private field
}

impl BankAccount {
    // Public constructor
    pub fn new(account_number: String, initial_balance: f64) -> Self {
        BankAccount {
            balance: initial_balance,
            account_number,
        }
    }

    // Public method to get balance (read-only access)
    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    // Public method to deposit
    pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    // Public method to withdraw
    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }
}

// 2. INHERITANCE-LIKE BEHAVIOR using traits
pub trait InterestBearing {
    fn calculate_interest(&self) -> f64;
    fn apply_interest(&mut self);
}

// SavingsAccount inherits BankAccount behavior and adds interest functionality
pub struct SavingsAccount {
    account: BankAccount,
    interest_rate: f64,
}

impl SavingsAccount {
    pub fn new(account_number: String, initial_balance: f64, interest_rate: f64) -> Self {
        SavingsAccount {
            account: BankAccount::new(account_number, initial_balance),
            interest_rate,
        }
    }

    // Delegate to inner account methods
    pub fn get_balance(&self) -> f64 {
        self.account.get_balance()
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
        self.account.deposit(amount)
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        self.account.withdraw(amount)
    }
}

impl InterestBearing for SavingsAccount {
    fn calculate_interest(&self) -> f64 {
        self.account.get_balance() * self.interest_rate
    }

    fn apply_interest(&mut self) {
        let interest = self.calculate_interest();
        let _ = self.account.deposit(interest);
    }
}

// 3. POLYMORPHISM with trait objects
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn describe(&self) -> String;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn describe(&self) -> String {
        format!("Rectangle: {}x{}", self.width, self.height)
    }
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn describe(&self) -> String {
        format!("Circle with radius: {}", self.radius)
    }
}

// 4. USING TRAIT OBJECTS FOR RUNTIME POLYMORPHISM
fn print_shape_info(shape: &dyn Shape) {
    println!("{}", shape.describe());
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}", shape.perimeter());
    println!();
}

fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

fn main() {
    println!("=== BANK ACCOUNT EXAMPLE (Encapsulation) ===");

    let mut account = BankAccount::new("12345".to_string(), 1000.0);
    println!("Initial balance: ${:.2}", account.get_balance());

    match account.deposit(500.0) {
        Ok(()) => println!("Deposited $500.00"),
        Err(e) => println!("Deposit error: {}", e),
    }

    match account.withdraw(200.0) {
        Ok(()) => println!("Withdrew $200.00"),
        Err(e) => println!("Withdrawal error: {}", e),
    }

    println!("Final balance: ${:.2}", account.get_balance());

    println!("\n=== SAVINGS ACCOUNT EXAMPLE (Inheritance-like) ===");

    let mut savings = SavingsAccount::new("67890".to_string(), 1000.0, 0.05);
    println!("Savings balance: ${:.2}", savings.get_balance());
    println!(
        "Interest to be applied: ${:.2}",
        savings.calculate_interest()
    );

    savings.apply_interest();
    println!("Balance after interest: ${:.2}", savings.get_balance());

    println!("\n=== SHAPES EXAMPLE (Polymorphism) ===");

    let rectangle = Rectangle::new(5.0, 3.0);
    let circle = Circle::new(4.0);

    // Using trait objects for runtime polymorphism
    print_shape_info(&rectangle);
    print_shape_info(&circle);

    // Collection of different shapes
    let shapes: Vec<&dyn Shape> = vec![&rectangle, &circle];
    let total = total_area(&shapes);
    println!("Total area of all shapes: {:.2}", total);

    println!("\n=== ADVANCED EXAMPLE: Animal Kingdom ===");

    // Another polymorphism example
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: "Buddy".to_string(),
        }),
        Box::new(Cat {
            name: "Whiskers".to_string(),
        }),
    ];

    for animal in animals {
        animal.speak();
        println!("{} says: {}", animal.get_name(), animal.make_sound());
    }
}

// Additional example: Classic OOP with animals
trait Animal {
    fn speak(&self);
    fn get_name(&self) -> &str;
    fn make_sound(&self) -> String;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} the dog says: Woof!", self.name);
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} the cat says: Meow!", self.name);
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }
}

