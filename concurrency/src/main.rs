use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
    println!("=== Fearless Concurrency Examples ===\n");

    // 1. Basic thread spawning
    basic_threads();
    
    // 2. Message passing with channels
    message_passing();
    
    // 3. Shared state with Arc and Mutex
    shared_state();
    
    // 4. Threads with move closures
    move_closures();
    
    // 5. Parallel computation with scoped threads
    parallel_computation();
}

fn basic_threads() {
    println!("1. Basic Thread Spawning:");
    
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: counting {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..=3 {
        println!("Main: counting {}", i);
        thread::sleep(Duration::from_millis(300));
    }

    handle.join().unwrap();
    println!("Basic threads completed!\n");
}

fn message_passing() {
    println!("2. Message Passing with Channels:");
    
    // Multiple Producer, Single Consumer channel
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // Clone the transmitter
    
    // Producer thread 1
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "thread", "1"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    // Producer thread 2
    thread::spawn(move || {
        let messages = vec!["Greetings", "from", "thread", "2"];
        for msg in messages {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });
    
    // Main thread receives messages
    for received in rx {
        println!("Received: {}", received);
    }
    println!("Message passing completed!\n");
}

fn shared_state() {
    println!("3. Shared State with Arc and Mutex:");
    
    // Arc = Atomic Reference Counting (thread-safe Rc)
    // Mutex = Mutual Exclusion for safe access
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
    println!("Shared state completed!\n");
}

fn move_closures() {
    println!("4. Move Closures with Threads:");
    
    let data = vec![1, 2, 3, 4, 5];
    
    // The 'move' keyword forces the closure to take ownership of `data`
    let handle = thread::spawn(move || {
        println!("Thread has data: {:?}", data);
        // data is moved here, can't be used in main thread anymore
        data.iter().sum::<i32>()
    });
    
    let result = handle.join().unwrap();
    println!("Sum calculated in thread: {}", result);
    println!("Move closures completed!\n");
}

fn parallel_computation() {
    println!("5. Parallel Computation:");
    
    let numbers: Vec<u64> = (1..=20).collect();
    let chunk_size = numbers.len() / 4;
    
    // Split work into chunks for parallel processing
    let chunks: Vec<Vec<u64>> = numbers.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    let mut handles = vec![];
    
    for chunk in chunks {
        let handle = thread::spawn(move || {
            // Simulate some CPU-intensive work
            let sum: u64 = chunk.iter().map(|&x| x * x).sum();
            (chunk.len(), sum)
        });
        handles.push(handle);
    }
    
    let mut total_sum = 0;
    let mut total_processed = 0;
    
    for handle in handles {
        let (processed, sum) = handle.join().unwrap();
        total_processed += processed;
        total_sum += sum;
        println!("Thread processed {} numbers, sum: {}", processed, sum);
    }
    
    println!("Total: processed {} numbers, final sum: {}", total_processed, total_sum);
    println!("Parallel computation completed!\n");
}
