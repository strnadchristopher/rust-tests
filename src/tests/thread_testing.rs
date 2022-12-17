use std::mem::size_of_val;
// Use threads
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
fn main(){
    // Create thread and store in variable
    let nums_to_generate = 100000;
    let mut fastest_time:Duration = Duration::new(0,0);
    let mut fastest_thread_count:i32 = 0;
    for num_of_threads in 1..100{
        // Get current time
        let start = std::time::Instant::now();
        start_test(num_of_threads, nums_to_generate);
        // Get time elapsed
        let elapsed = start.elapsed();
        // Print time elapsed, and number of threads
        println!("{} threads took {} milliseconds", num_of_threads, elapsed.as_millis());
        if(fastest_time == Duration::new(0,0) || elapsed < fastest_time){
            fastest_time = elapsed;
            fastest_thread_count = num_of_threads;
        }
        else if (elapsed > fastest_time){
        }
    }
    // Print fastest time and fastest thread_count
    println!("Fastest time was {} milliseconds with {} threads", fastest_time.as_millis(), fastest_thread_count);
}

fn start_test(num_of_threads:i32, nums_to_generate:i32){
    // Create an resizable array
    let mut array: Vec<i32> = Vec::new();
    let chunk_size = nums_to_generate / num_of_threads;
    let mut handles: Vec<JoinHandle<Vec<i32>>> = Vec::new();
    for thread_number in 4..num_of_threads{
        println!("Starting thread {}", thread_number);
        handles.push(start_thread(chunk_size, thread_number));
    }

    for handle in handles{
        array.append(&mut handle.join().unwrap());
    }
    let mut final_array: Vec<i32> = Vec::new();
    for i_array in &array{
        final_array.push(*i_array);
    }

    // array = slow_method();
    // Sort the array
    final_array.sort();

    // Print array
    // println!("{:?}", array);

    // println!("Array created");
    // Wait for user input
    // Print array
    // println!("{:?}", array);
}

fn start_thread(chunk_size:i32, thread_number:i32) -> thread::JoinHandle<Vec<i32>>{
    // Create thread and store in variable
    let thread_handle = std::thread::spawn(move || {
        // Push 100000 random numbers into the array
        let mut new_array: Vec<i32> = Vec::new();
        for _ in 0..chunk_size{
            // store a random number between 1 and 10000
            let random_number = rand::random::<i32>();
            // Add random number with 5
            let new_number = random_number + 5;
            new_array.push(new_number);
            // println!("Thread {} is at {}", thread_number, new_array.len());
            // print!("\r")
        }
        println!("Thread {} finished", thread_number);
        new_array
    });

    // Wait for thread to finish
    thread_handle
}

fn slow_method() -> Vec<i32>{
    // Push 100000 random numbers into the array
    let mut new_array: Vec<i32> = Vec::new();
    for _ in 0..10000000{
        new_array.push(rand::random::<i32>());
    }
    new_array
}