use std::collections::HashMap;

fn main() {
    println!("=== VECTOR EXAMPLES ===");
    vector_examples();

    println!("\n=== STRING EXAMPLES ===");
    string_examples();

    println!("\n=== HASHMAP EXAMPLES ===");
    hashmap_examples();

    println!("\n=== PRACTICAL EXAMPLE COMBINING ALL THREE ===");
    practical_example();
}

fn vector_examples() {
    // Creating vectors
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1: {:?}", v1);

    // Using vec! macro
    let v2 = vec![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);

    // Accessing elements
    let third = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // Safe access with get
    match v2.get(99) {
        Some(value) => println!("Element at index 99: {}", value),
        None => println!("No element at index 99"),
    }

    // Iterating
    print!("Iterating v2: ");
    for i in &v2 {
        print!("{} ", i);
    }
    println!();

    // Iterating with mutable references
    let mut v3 = vec![100, 32, 57];
    print!("v3 before: {:?}", v3);
    for i in &mut v3 {
        *i += 50;
    }
    println!(", after: {:?}", v3);

    // Using enums to store different types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Row with different types: {:?}", row);

    // Vector methods
    println!("v2 length: {}", v2.len());
    println!("v2 capacity: {}", v2.capacity());

    // Removing elements
    let mut v4 = vec![1, 2, 3, 4, 5];
    let removed = v4.remove(2); // Remove element at index 2
    println!("Removed {} from v4: {:?}", removed, v4);

    // Popping from end
    let last = v4.pop();
    println!("Popped {:?} from v4: {:?}", last, v4);
}

fn string_examples() {
    // Creating strings
    let mut s1 = String::new();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    println!("s1: '{}'", s1);
    println!("s2: '{}'", s2);
    println!("s3: '{}'", s3);

    // Updating strings
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("After updates: '{}'", s);

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here
    println!("Concatenated: '{}'", s3);
    // println!("{}", s1); // This would error - s1 was moved

    // Using format! macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Formatted: '{}'", s);
    println!("s1 still accessible: '{}'", s1); // format! doesn't take ownership

    // Slicing strings (be careful with UTF-8!)
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Each Cyrillic character is 2 bytes
    println!("Sliced '{}' to get: '{}'", hello, s);

    // Iterating over strings
    let hindi = "नमस्ते";
    print!("Characters in '{}': ", hindi);
    for c in hindi.chars() {
        print!("{} ", c);
    }
    println!();

    print!("Bytes in '{}': ", hindi);
    for b in hindi.bytes() {
        print!("{} ", b);
    }
    println!();

    // String methods
    let s = String::from("hello world");
    println!("'{}' length: {}", s, s.len());
    println!("'{}' is empty: {}", s, s.is_empty());
    println!("'{}' contains 'world': {}", s, s.contains("world"));

    // Replacing parts of string
    let replaced = s.replace("world", "Rust");
    println!("After replace: '{}'", replaced);

    // Splitting
    let sentence = "The quick brown fox";
    print!("Words in '{}': ", sentence);
    for word in sentence.split_whitespace() {
        print!("{} ", word);
    }
    println!();
}

fn hashmap_examples() {
    // Creating HashMaps
    let mut scores = HashMap::new();

    // Inserting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Initial scores: {:?}", scores);

    // Creating from vectors using collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Scores from vectors: {:?}", scores2);

    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{} team score: {}", team_name, s),
        None => println!("{} team not found", team_name),
    }

    // Accessing non-existent key
    let unknown_team = String::from("Red");
    let score = scores.get(&unknown_team);
    match score {
        Some(s) => println!("{} team score: {}", unknown_team, s),
        None => println!("{} team not found", unknown_team),
    }

    // Iterating
    println!("All teams and scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // Updating values
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("Before overwrite: {:?}", scores);

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("After overwrite: {:?}", scores);

    // Only inserting if key has no value using entry API
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Won't change Blue's value
    println!("After entry operations: {:?}", scores);

    // Updating based on old value - word count example
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", map);

    // HashMap ownership demonstration
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point!
    println!("Ownership demo map: {:?}", map);
}

fn practical_example() {
    #[derive(Debug)]
    struct StudentManager {
        students: HashMap<String, Vec<String>>, // name -> courses
    }

    impl StudentManager {
        fn new() -> Self {
            Self {
                students: HashMap::new(),
            }
        }

        fn add_student(&mut self, name: String, courses: Vec<String>) {
            self.students.insert(name, courses);
        }

        fn get_courses(&self, name: &str) -> Option<&Vec<String>> {
            self.students.get(name)
        }

        fn add_course(&mut self, name: &str, course: String) {
            self.students
                .entry(name.to_string())
                .or_insert_with(Vec::new)
                .push(course);
        }

        fn list_all_students(&self) -> String {
            let mut result = String::new();

            for (name, courses) in &self.students {
                result.push_str(&format!("{}: ", name));
                result.push_str(&courses.join(", "));
                result.push('\n');
            }

            result
        }

        fn get_student_count(&self) -> usize {
            self.students.len()
        }
    }

    let mut manager = StudentManager::new();

    // Add students with their courses
    manager.add_student(
        "Alice".to_string(),
        vec!["Math".to_string(), "Science".to_string()],
    );

    manager.add_student(
        "Bob".to_string(),
        vec!["History".to_string(), "Art".to_string()],
    );

    // Add a course to existing student
    manager.add_course("Alice", "Programming".to_string());

    // Add a new student with no courses initially
    manager.add_course("Charlie", "Physics".to_string());

    println!("Student Manager Contents:");
    println!("{}", manager.list_all_students());
    println!("Total students: {}", manager.get_student_count());

    // Look up specific student
    if let Some(courses) = manager.get_courses("Alice") {
        println!("Alice's courses: {:?}", courses);
    }

    if let Some(courses) = manager.get_courses("David") {
        println!("David's courses: {:?}", courses);
    } else {
        println!("David is not in the system");
    }

    // Demonstrate all three collections working together
    let class_rosters: HashMap<String, Vec<String>> = manager.students;
    println!("\nDirect access to HashMap: {:?}", class_rosters);
}

