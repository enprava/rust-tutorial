// main.rs

// ===== BASIC ENUMS =====
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// ===== ENUMS WITH DATA =====
#[derive(Debug)]
enum IpAddrAdvanced {
    V4(u8, u8, u8, u8), // IPv4 as four u8 values
    V6(String),         // IPv6 as a string
}

// ===== ENUMS WITH NAMED FIELDS =====
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields like a struct
    Write(String),              // Single String
    ChangeColor(i32, i32, i32), // Three i32 values
}

// Implementing methods on enums
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

// ===== THE OPTION ENUM =====
fn process_number(num: Option<i32>) -> Option<i32> {
    match num {
        Some(n) => {
            println!("Processing number: {}", n);
            Some(n * 2)
        }
        None => {
            println!("No number provided");
            None
        }
    }
}

// ===== MATCH EXPRESSIONS =====
fn match_demo(value: u8) {
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Something else: {}", value), // Catch-all pattern
    }
}

// Match with enums
fn match_ip_kind(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("It's an IPv4 address"),
        IpAddrKind::V6 => println!("It's an IPv6 address"),
    }
}

// ===== IF LET SYNTAX =====
fn if_let_demo(value: Option<i32>) {
    // Using match (more verbose)
    match value {
        Some(7) => println!("Lucky number seven!"),
        _ => (),
    }

    // Using if let (more concise)
    if let Some(7) = value {
        println!("Lucky number seven with if let!");
    }

    // if let with else
    if let Some(x) = value {
        println!("Got a value: {}", x);
    } else {
        println!("Got nothing!");
    }
}

// ===== PRACTICAL EXAMPLE: USER ROLES =====
#[derive(Debug)]
enum UserRole {
    Guest,
    User {
        username: String,
        email: String,
    },
    Admin {
        username: String,
        permissions: Vec<String>,
    },
}

impl UserRole {
    fn get_description(&self) -> String {
        match self {
            UserRole::Guest => "Guest user with limited access".to_string(),
            UserRole::User { username, email } => {
                format!("Regular user: {} ({})", username, email)
            }
            UserRole::Admin {
                username,
                permissions,
            } => {
                format!(
                    "Admin user: {} with permissions: {:?}",
                    username, permissions
                )
            }
        }
    }

    fn can_access(&self, resource: &str) -> bool {
        match self {
            UserRole::Guest => resource == "public",
            UserRole::User { .. } => resource == "public" || resource == "user_data",
            UserRole::Admin { .. } => true, // Admins can access everything
        }
    }
}

fn main() {
    println!("=== BASIC ENUMS ===");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IP kinds: {:?}, {:?}", four, six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("Home IP: {:?}", home);

    println!("\n=== ENUMS WITH DATA ===");
    let home_advanced = IpAddrAdvanced::V4(127, 0, 0, 1);
    let loopback_advanced = IpAddrAdvanced::V6(String::from("::1"));
    println!("Advanced IPs: {:?}, {:?}", home_advanced, loopback_advanced);

    println!("\n=== ENUMS WITH METHODS ===");
    let msg1 = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::ChangeColor(255, 0, 0);
    let msg4 = Message::Quit;

    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    println!("\n=== OPTION ENUM ===");
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);

    let doubled = process_number(some_number);
    println!("Doubled: {:?}", doubled);

    let nothing = process_number(no_number);
    println!("Nothing: {:?}", nothing);

    println!("\n=== MATCH EXPRESSIONS ===");
    match_demo(3);
    match_demo(7);
    match_ip_kind(IpAddrKind::V4);
    match_ip_kind(IpAddrKind::V6);

    // Match with Option
    let five = Some(5);
    match five {
        Some(n) if n > 3 => println!("Number is greater than 3: {}", n),
        Some(n) => println!("Number is 3 or less: {}", n),
        None => println!("No number"),
    }

    println!("\n=== IF LET SYNTAX ===");
    if_let_demo(Some(7));
    if_let_demo(Some(10));
    if_let_demo(None);

    println!("\n=== PRACTICAL EXAMPLE: USER ROLES ===");
    let guest = UserRole::Guest;
    let user = UserRole::User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
    };
    let admin = UserRole::Admin {
        username: String::from("bob"),
        permissions: vec![
            String::from("read"),
            String::from("write"),
            String::from("delete"),
        ],
    };

    let users = vec![guest, user, admin];

    for user in users {
        println!("User: {:?}", user);
        println!("Description: {}", user.get_description());
        println!("Can access 'public': {}", user.can_access("public"));
        println!("Can access 'user_data': {}", user.can_access("user_data"));
        println!(
            "Can access 'admin_panel': {}",
            user.can_access("admin_panel")
        );
        println!("---");
    }

    println!("\n=== MATCH WITH PATTERNS ===");
    // Destructuring in matches
    let msg = Message::Move { x: 10, y: 20 };
    if let Message::Move { x, y } = msg {
        println!("Moving to coordinates: x={}, y={}", x, y)
    }

    // Multiple patterns
    let value = 2;
    match value {
        1 | 2 => println!("One or two"),
        3..=5 => println!("Three to five"),
        _ => println!("Other"),
    }
}
