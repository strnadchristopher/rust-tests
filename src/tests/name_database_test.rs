use std::io;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use rand::Rng;
use std::thread;
#[derive(Serialize, Deserialize, Debug)]
struct User{
    name: String,
    age: u8,
    party_size: u8,
    turn_time: u8,
    reservation_time: i32,
}

struct NameList{
    json_path: &'static str,
    names_as_string: String,
    names: Value,
}

fn main(){
    let mut first_name_list = NameList{
        json_path:"./json/first-names.json",
        names_as_string: std::fs::read_to_string("./json/first-names.json").unwrap(),
    };

    first_name_list.names = serde_json::from_str::<Value>(first_name_list.names_as_string).unwrap();

    // Get the path to the JSON file.
    const last_names_input_path:&str = "./json/last-names.json";

    // Load the files into strings.
    const first_names_string : Value = {
        // Load the first file into a string.
        let text = std::fs::read_to_string(&first_names_input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    let mut last_names_string: Value = {
        // Load the first file into a string.
        let text = std::fs::read_to_string(&last_names_input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    // Convert the JSON structure into an array.
    let first_names = first_names_string.as_array_mut().unwrap();
    let last_names = last_names_string.as_array_mut().unwrap();

    // Get the length of the array.
    let first_names_array_length = first_names.len();
    let last_names_array_length = last_names.len();

    // Set user count constant
    const USER_COUNT: i32 = 1000000;
    const THREAD_COUNT: i32 = 4;
    
    let list_size = USER_COUNT / THREAD_COUNT;

    let mut thread_list = vec![];
    let mut user_lists = vec![];
    
    // For every thread, generate a user list and save it to a file
    thread_list.push(thread::spawn(move || unsafe{
        user_lists.push(generate_user_list(list_size,first_names, last_names, first_names_array_length, last_names_array_length));
    }));



    for thread in thread_list{
        let _ = thread.join();
    }

    // for user_list in user_lists{
    //     println!("{:?}", user_list);
    // }

    // Combine all lists in user_lists into one list

    let mut final_user_list: Vec<User> = Vec::new();
    for user_list in user_lists{
        for user in user_list{
            final_user_list.push(user);
        }
    }

    // // Print the user list
    // for user in &user_list{
    //     println!("{:?}", user);
    // }

    // Convert user_list to json string serialized
    let user_list_json = serde_json::to_string(&final_user_list).unwrap();
    
    println!("Saving user list to file...");
    // Save user_list_json to file in new thread
    let save_thread_handle = thread::spawn(|| {
        std::fs::write("./json/user-list.json", user_list_json).expect("Unable to write file");
        println!("Saved user list to file");
    });

    // Wait for save thread to finish
    save_thread_handle.join().unwrap();

    
}

fn generate_user_list(list_size: i32, first_names : &mut Vec<Value>, last_names : &mut Vec<Value>, first_names_length : usize, last_names_length : usize) -> Vec<User>{
    let mut user_list: Vec<User> = Vec::new();
    for user_number in 0..list_size{
        user_list.push(generate_user(user_number, first_names, last_names, first_names_length, last_names_length));
    }
    user_list
}

fn generate_user(user_number: i32, first_names : &mut Vec<Value>, last_names : &mut Vec<Value>, first_names_length : usize, last_names_length : usize) -> User{
    // Randomly generate new user
    //Initialize random number generator
    let mut rng = rand::thread_rng();

    let new_name = format!("{} {}", first_names[rng.gen_range(0..first_names_length)], last_names[rng.gen_range(0..last_names_length)]);
    let new_age = rng.gen_range(0..100);
    let new_party_size = rng.gen_range(1..8);
    
    // Set new_turn_time to a random number between 45 and 180 if new_party_size is between 1 and 4, otherwise set it to a random number between 120 and 240
    let new_turn_time = if new_party_size > 0 && new_party_size < 5{
        rng.gen_range(45..180)
    }else{
        rng.gen_range(120..240)
    };

    let new_user = User{
        name: new_name,
        age: new_age,
        party_size: rng.gen_range(0..8),
        turn_time: new_turn_time,
        reservation_time: rng.gen_range(0..500),
    };
    println!("{}", user_number);
    // println!("{:?}", new_user);
    new_user
}

fn get_input(prompt : &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim_end().to_string()
}
