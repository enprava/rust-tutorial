// main.rs

// ===== GENERIC TYPES =====

// 1. Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 2. Generic struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

// 3. Generic struct with multiple type parameters
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}

// ===== TRAITS =====

// 4. Defining traits
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn summary_default(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Display {
    fn display(&self) -> String;
}

// 5. Implementing traits for structs
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 6. Trait bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn notify_multiple<T: Summary + Display>(item: &T) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item.display());
}

// Alternative syntax with where clause
fn notify_where<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Where clause - Summary: {}", item.summarize());
}

// 7. Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// ===== LIFETIMES =====

// 8. Lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 9. Lifetime in structs
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

// 10. Static lifetime
fn static_lifetime() -> &'static str {
    "I have a static lifetime!"
}

// ===== COMBINING ALL CONCEPTS =====

// Generic function with trait bounds and lifetimes
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("=== GENERIC TYPES EXAMPLES ===");

    // Generic function usage
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Generic struct usage
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    println!(
        "Integer point: ({}, {})",
        integer_point.x(),
        integer_point.y
    );
    println!("Float point: ({}, {})", float_point.x, float_point.y);

    let mixed_pair = Pair::new(5, "hello");
    println!("Mixed pair: {}, {}", mixed_pair.first, mixed_pair.second);

    println!("\n=== TRAITS EXAMPLES ===");

    // Traits usage
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust 1.0 is here!"),
        reply: false,
        retweet: false,
    };

    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    println!("Default summary: {}", article.summary_default());

    // Trait bounds
    notify(&article);
    notify(&tweet);

    // Returning trait objects
    let summarizable = returns_summarizable();
    println!("Returned summarizable: {}", summarizable.summarize());

    println!("\n=== LIFETIMES EXAMPLES ===");

    // Lifetimes
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt part: {}", excerpt.part);
    println!("Excerpt level: {}", excerpt.level());

    let part = excerpt.announce_and_return_part("This is important!");
    println!("Returned part: {}", part);

    // Static lifetime
    let s: &'static str = static_lifetime();
    println!("Static string: {}", s);

    println!("\n=== COMBINED EXAMPLE ===");

    // Combined example
    let result = longest_with_announcement("long string", "short", "Today's announcement!");
    println!("Longest with announcement: {}", result);

    // Demonstrating lifetime constraints
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // This would cause a compile error if we tried to use result here
    // after string2 goes out of scope, demonstrating lifetime safety

    println!("\nAll examples completed successfully!");
}

