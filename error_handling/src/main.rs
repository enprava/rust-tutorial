fn main() {
    println!("=== Rust Error Handling Examples ===\n");

    // Example 1: Unrecoverable Errors with panic!
    println!("1. Unrecoverable Errors with panic!");
    // Uncomment the line below to see panic in action
    // panic_example();
    conditional_panic_example();
    println!();

    // Example 2: Recoverable Errors with Result
    println!("2. Recoverable Errors with Result");
    result_examples();
    println!();

    // Example 3: Propagating Errors
    println!("3. Propagating Errors");
    match read_and_validate_number("42") {
        Ok(num) => println!("Successfully parsed: {}", num),
        Err(e) => println!("Error: {}", e),
    }
    match read_and_validate_number("not_a_number") {
        Ok(num) => println!("Successfully parsed: {}", num),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 4: Using ? operator for error propagation
    println!("4. Using ? operator");
    match read_config_file() {
        Ok(config) => println!("Config content: {}", config),
        Err(e) => println!("Failed to read config: {}", e),
    }
    println!();

    // Example 5: Custom error types
    println!("5. Custom Error Types");
    match process_user_input("hello") {
        Ok(result) => println!("Processing result: {}", result),
        Err(e) => println!("Custom error: {:?}", e),
    }
    match process_user_input("") {
        Ok(result) => println!("Processing result: {}", result),
        Err(e) => println!("Custom error: {:?}", e),
    }

    additional_examples();
    panic_example();
}

// ===== PANIC EXAMPLES =====

fn panic_example() {
    // This will immediately terminate the program
    panic!("This is a deliberate panic!");
}

fn conditional_panic_example() {
    let should_panic = false; // Change to true to see panic

    if should_panic {
        panic!("Conditional panic occurred!");
    } else {
        println!("No panic this time!");
    }
}

// ===== RESULT EXAMPLES =====

fn result_examples() {
    // Basic Result matching
    let result: Result<i32, &str> = Ok(42);

    match result {
        Ok(value) => println!("Got value: {}", value),
        Err(error) => println!("Got error: {}", error),
    }

    // Using unwrap and expect
    let success_result: Result<i32, &str> = Ok(100);
    let value = success_result.unwrap(); // Extracts value if Ok, panics if Err
    println!("Unwrapped value: {}", value);

    let success_with_expect: Result<i32, &str> = Ok(200);
    let value = success_with_expect.expect("This should never fail!");
    println!("Expected value: {}", value);

    // Handling potential failure
    let failure_result: Result<i32, &str> = Err("Something went wrong");

    // Using unwrap_or to provide default value
    let default_value = failure_result.unwrap_or(0);
    println!("Default value after error: {}", default_value);

    // Using if let for simple matching
    if let Ok(num) = "42".parse::<i32>() {
        println!("Parsed number: {}", num);
    }
}

// ===== ERROR PROPAGATION EXAMPLES =====

fn read_and_validate_number(input: &str) -> Result<i32, String> {
    // Parse the string to i32
    let number: i32 = input
        .parse()
        .map_err(|_| format!("Failed to parse '{}' as number", input))?;

    // Validate the number
    if number < 0 {
        return Err("Number must be positive".to_string());
    }

    if number > 100 {
        return Err("Number must be less than or equal to 100".to_string());
    }

    Ok(number)
}

// Function that simulates reading a file
fn read_file_contents(filename: &str) -> Result<String, String> {
    match filename {
        "config.txt" => Ok("database_url=localhost:5432".to_string()),
        "settings.txt" => Ok("theme=dark".to_string()),
        _ => Err(format!("File '{}' not found", filename)),
    }
}

fn read_config_file() -> Result<String, String> {
    // The ? operator automatically returns the error if it occurs
    let config = read_file_contents("config.txt")?;
    let settings = read_file_contents("settings.txt")?;

    Ok(format!("Config: {}, Settings: {}", config, settings))
}

// ===== CUSTOM ERROR TYPE EXAMPLE =====

#[derive(Debug)]
enum ProcessingError {
    EmptyInput,
    InvalidFormat,
    TooLong,
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProcessingError::EmptyInput => write!(f, "Input cannot be empty"),
            ProcessingError::InvalidFormat => write!(f, "Input has invalid format"),
            ProcessingError::TooLong => write!(f, "Input is too long"),
        }
    }
}

fn process_user_input(input: &str) -> Result<String, ProcessingError> {
    if input.is_empty() {
        return Err(ProcessingError::EmptyInput);
    }

    if input.len() > 10 {
        return Err(ProcessingError::TooLong);
    }

    if !input.chars().all(|c| c.is_alphabetic()) {
        return Err(ProcessingError::InvalidFormat);
    }

    Ok(input.to_uppercase())
}

// ===== ADDITIONAL UTILITY EXAMPLES =====

fn additional_examples() {
    println!("\n=== Additional Examples ===");

    // Using map and and_then with Result
    let number_result: Result<i32, &str> = Ok(5);

    let doubled = number_result.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);

    let chained = number_result.and_then(|n| {
        if n > 0 {
            Ok(n * 2)
        } else {
            Err("Number must be positive")
        }
    });
    println!("Chained: {:?}", chained);

    // Combining multiple Results
    let result1: Result<i32, &str> = Ok(1);
    let result2: Result<i32, &str> = Ok(2);
    let result3: Result<i32, &str> = Ok(3);

    let combined = result1.and_then(|a| result2.and_then(|b| result3.map(|c| a + b + c)));
    println!("Combined: {:?}", combined);
}
