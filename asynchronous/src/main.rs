// main.rs
use std::time::Duration;
use tokio::time::sleep;

// Basic async function
async fn hello_world() {
    println!("Hello, async world!");
}

// Async function with delay
async fn delayed_greeting(name: &str) {
    println!("Starting delayed greeting for {}", name);
    sleep(Duration::from_secs(2)).await;
    println!("Hello, {}! (after 2 seconds)", name);
}

// Async function that returns a value
async fn add_async(a: i32, b: i32) -> i32 {
    sleep(Duration::from_secs(1)).await;
    a + b
}

// Multiple async operations
async fn multiple_operations() {
    println!("\n--- Running multiple operations sequentially ---");
    
    // These run one after another
    let result1 = add_async(5, 3).await;
    let result2 = add_async(10, 7).await;
    println!("Sequential results: {} and {}", result1, result2);
}

// Concurrent async operations
async fn concurrent_operations() {
    println!("\n--- Running operations concurrently ---");
    
    // These start at the same time
    let task1 = add_async(5, 3);
    let task2 = add_async(10, 7);
    
    // Wait for both to complete
    let (result1, result2) = tokio::join!(task1, task2);
    println!("Concurrent results: {} and {}", result1, result2);
}

// Async function with error handling
async fn divide_async(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        sleep(Duration::from_secs(1)).await;
        Ok(a / b)
    }
}

// Working with vectors of async tasks
async fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let mut handles = vec![];
    
    for &num in &numbers {
        // Spawn tasks concurrently
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(100)).await;
            num * 2
        });
        handles.push(handle);
    }
    
    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    
    results
}

#[tokio::main]
async fn main() {
    println!("=== Rust Async/Await Examples ===\n");
    
    // Basic async call
    hello_world().await;
    
    // Simple delayed operation
    delayed_greeting("Alice").await;
    
    // Async function with return value
    let sum = add_async(10, 20).await;
    println!("\n10 + 20 = {}", sum);
    
    // Sequential vs concurrent execution
    multiple_operations().await;
    concurrent_operations().await;
    
    // Error handling in async
    match divide_async(10.0, 2.0).await {
        Ok(result) => println!("\n10.0 / 2.0 = {}", result),
        Err(e) => println!("\nError: {}", e),
    }
    
    match divide_async(10.0, 0.0).await {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Processing multiple items concurrently
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = process_numbers(numbers).await;
    println!("\nDoubled numbers: {:?}", doubled);
    
    println!("\n=== All examples completed ===");
}
