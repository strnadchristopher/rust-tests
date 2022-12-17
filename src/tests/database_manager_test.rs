// Use random
use rand::Rng;
// Use serde_json
// use serde_json::json;
use serde::{Deserialize, Serialize};
use terminal_menu::{button, label, menu, mut_menu, run};
mod database_manager;


#[derive(Debug, Serialize, Deserialize, Clone)]
enum DataType{
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

// A program that creates a database
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Database {
    name: String,
    tables: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Table {
    name: String,
    columns: Vec<Column>,
    rows: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Column {
    title: String,
    data_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User{
    name: String,
    age: i32,
}


fn main() {
    // Set current state to MainMenu
    let mut database = Database {
        name: String::from("Database"),
        tables: Vec::new(),
    };
    let main_menu_choice = main_menu();
    match main_menu_choice.as_str(){
        "Load Database" => {
            database = load_database();
        }
        "Create Database" => {
            database = create_database();
        }
        "Exit" => exit(),
        _ => println!("Invalid input"),
    }

    // Print database
    // println!("{:?}", database);
    
    let database_manager_choice = database_manager_menu(&database);
    // If database_manager_choice contains "View"
    if database_manager_choice.contains("View"){
        // Get table name
        let table_name = database_manager_choice.replace("View ", "");
        // Get table
        let table = get_table(&database, table_name);

        // Display table as a menu
        let table_menu_choice = table_menu(&table);
    }
}

fn main_menu() -> String {
    
    let menu = menu(vec![
        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("Rust Database Manager"),
        label("----------------------"),
        label("use wasd or arrow keys"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------------"),
        // button:
        //  exit the menu
        button("Load Database"),
        button("Create Database"),
        button("Exit"),
    ]);
    run(&menu);
    let choice = mut_menu(&menu).selected_item_name().to_string();
    choice
}

fn database_manager_menu(database:&Database) -> String {
    // Show menu with database name and tables
    let mut menu_items = Vec::new();
    menu_items.push(label("----------------------"));
    menu_items.push(label("Database Menu"));
    menu_items.push(label("----------------------"));
    menu_items.push(label(&format!("Database: {}", database.name)));
    menu_items.push(label("----------------------"));
    menu_items.push(label("use wasd or arrow keys"));
    menu_items.push(label("enter to select"));
    menu_items.push(label("'q' or esc to exit"));
    menu_items.push(label("-----------------------"));
    for table in &database.tables {
        // Push button with text "View " + table name
        menu_items.push(button(&format!("View {}", table.name)));
    }
    menu_items.push(button("Create Table"));
    menu_items.push(button("Exit"));
    let menu = menu(menu_items);
    run(&menu);

    let choice = mut_menu(&menu).selected_item_name().to_string();
    choice
}

// Get table from database
fn get_table(database: &Database, table_name: String) -> Table{
    let table = Table {
        name: String::from(""),
        columns: Vec::new(),
        rows: Vec::new(),
    };
    for table in &database.tables{
        if table.name == table_name{
            return table.clone();
        }
    }
    table
}

fn table_menu(table: &Table) -> String{
    // Show menu with table name and columns
    let mut menu_items = Vec::new();
    menu_items.push(label("----------------------"));
    menu_items.push(label(&format!("{}", table.name)));
    
    // Create string of column names
    let mut column_names_string = String::from("");
    for column in &table.columns {
        column_names_string.push_str(&format!("{}    ", column.title));
    }
    menu_items.push(label(&format!("{}", column_names_string)));

    // Create string of rows for every user
    for user in &table.rows{
        let mut user_string = String::from("");
        for column in &table.columns{
            match column.data_type.as_str(){
                "Int" => {
                    user_string.push_str(&format!("{}    ", user.age));
                }
                "String" => {
                    user_string.push_str(&format!("{}    ", user.name));
                }
                _ => println!("Invalid data type"),
            }
        }
        menu_items.push(label(&format!("{}", user_string)));
    }
    
    
    menu_items.push(label("-----------------------"));
    menu_items.push(button("Create Column"));
    menu_items.push(button("Back"));
    let menu = menu(menu_items);
    run(&menu);

    let choice = mut_menu(&menu).selected_item_name().to_string();
    choice
}

fn load_database() -> Database{
    let database_name = String::from("Database");
    if std::path::Path::new(&format!("{}.db", database_name)).exists() {
        println!("Database file exists");
        // If database file exists, load it
        let database_json =
            std::fs::read_to_string(&format!("{}.db", database_name)).expect("Unable to read file");
        // Convert json string to database struct
        let database: Database = serde_json::from_str(&database_json).unwrap();
        // Print database
        // println!("{:?}", database);
        database
    }else{
        println!("Database file does not exist");
        // If database file does not exist, create it
        let database = create_database();
        save_database(database.clone());
        database
    }
}

fn create_database() -> Database{
    let mut database = Database {
        name: String::from("Database"),
        tables: Vec::new(),
    };

    let mut users_table = Table {
        name: String::from("Users"),
        columns: Vec::new(),
        rows: Vec::new(),
    };

    let name_column = Column {
        title: String::from("Name"),
        data_type: String::from("String"),
    };

    let age_column = Column {
        title: String::from("Age"),
        data_type: String::from("Int"),
    };

    // Create a list of ten users, where the name is "User " + i and the age is a random i32 between 13 and 78
    let mut rng = rand::thread_rng();
    let mut users = Vec::new();
    for i in 0..5{
        let user = User {
            name: format!("User {}", i),
            age: rng.gen_range(13..78)
        };
        users.push(user);
    }
    
    users_table.rows = users;
    users_table.columns.push(name_column);
    users_table.columns.push(age_column);
    database.tables.push(users_table);

    database

}

fn save_database(database: Database){
    // Convert database to json string
    let json_string = serde_json::to_string(&database).unwrap();

    // Save json string to file named (database.name).db
    std::fs::write(format!("{}.db", database.name), json_string).expect("Unable to write file");

    // Print success message
    println!("Database saved");
}
fn exit(){
    println!("Exiting...");
    std::process::exit(0);
}