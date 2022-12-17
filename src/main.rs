#[macro_use] extern crate rocket;
mod database_manager;
use rocket::State;

#[post("/query", data="<input>")]
fn query(database: &State<database_manager::Database>, input: String) -> String {
    // lowercase input string
    let input = input.to_lowercase();
    // If input starts with select, return select
    if input.starts_with("select") {
        return database_manager::select(database, input);
    }else if input.starts_with("insert") {
        return database_manager::insert(database, input);
    }else if input.starts_with("update") {
        // return database_manager::update(database, input);
    }else if input.starts_with("delete") {
        // return database_manager::delete(database, input);
    }

    return String::from("Invalid query");
}

#[get("/")]
fn index(database: &State<database_manager::Database>) -> String {
    // Convert database to string
    let database_string = database.to_string();
    // Convert database_string to &str static
    // Return database_str
    return database_string;
}


#[launch]
fn rocket() -> _ {
    // Load database
    let database = database_manager::load_database();

    println!("Loaded database, starting server");

    rocket::build()
    .mount("/", routes![query, index])
    .manage(database)
}