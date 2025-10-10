use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// Advanced Traits: Associated Types and Default Generic Parameters
trait Processor {
    type Input;
    type Output;

    fn process(&self, input: Self::Input) -> Self::Output;
}

struct StringProcessor;
impl Processor for StringProcessor {
    type Input = String;
    type Output = usize;

    fn process(&self, input: String) -> usize {
        input.len()
    }
}

// Advanced Traits: Operator Overloading
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Advanced Types: Newtype Pattern
struct Meters(f64);
struct Kilometers(f64);

impl Meters {
    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 / 1000.0)
    }
}

// Advanced Functions: Function Pointers and Closures
fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

// Unsafe Rust
unsafe fn dangerous_operation() -> *const i32 {
    let x = 42;
    &x as *const i32
}

// Advanced Lifetimes
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Async/Await (requires tokio in dependencies)
// Uncomment if you have tokio in your Cargo.toml
/*
async fn async_task() -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    "Async task completed".to_string()
}
*/

fn main() {
    println!("=== Advanced Traits Examples ===");

    // Associated Types
    let processor = StringProcessor;
    let result = processor.process("Hello, Rust!".to_string());
    println!("String length: {}", result);

    // Operator Overloading
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("Point addition: {:?}", p3);

    println!("\n=== Advanced Types ===");
    // Newtype Pattern
    let distance = Meters(1500.0);
    let km = distance.to_kilometers();
    println!("Distance: {:.2} km", km.0);

    println!("\n=== Advanced Functions ===");
    // Function Pointers and Closures
    let double = |x| x * 2;
    let result = apply_twice(double, 5);
    println!("Double twice of 5: {}", result);

    // Function pointer type
    let fn_ptr: fn(i32) -> i32 = |x| x + 1;
    println!("Function pointer result: {}", fn_ptr(10));

    println!("\n=== Concurrency Examples ===");

    // Message Passing
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec!["more", "messages", "here"];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Receive messages in main thread
    for received in rx {
        println!("Received: {}", received);
        if received == "here" {
            break;
        } // Early exit for demo
    }

    // Shared State Concurrency
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());

    println!("\n=== Advanced Lifetimes ===");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!(
        "Excerpt: {}",
        i.announce_and_return_part("Important message")
    );

    println!("\n=== Unsafe Rust ===");
    // Unsafe block
    unsafe {
        let ptr = dangerous_operation();
        println!("Dangerous operation result: {}", *ptr);
    }

    // Raw pointers
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    println!("\n=== Advanced Pattern Matching ===");
    // if let and while let
    let some_option_value = Some(5);

    if let Some(x) = some_option_value {
        println!("Found value in option: {}", x);
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    // Pattern matching with guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than 5: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    println!("\nAll examples completed!");
}

