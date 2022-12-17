use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
}
// A program that creates a database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub name: String,
    pub tables: Vec<Table>,
}
impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write all values of database and all children to f
        write!(f, "Database: {}\n\n", self.name)?;
        for table in &self.tables {
            write!(f, "{}\n", table)?;
        }
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write all values of table and all children to f
        write!(f, "Table: {}\n", self.name)?;
        for column in &self.columns {
            write!(f, "{}    ", column)?;
        }
        write!(f, "\n")?;
        for row in &self.rows {
            write!(f, "{}\n", row)?;
        }
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize, Clone,)]
pub struct Column {
    pub title: String,
    pub data_type: String,
}
impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write all values of column to f
        write!(f, "{}       ", self.title)?;
        Ok(())
    }
}
// Row is a vector of cells
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row{
    pub cells: Vec<Cell>,
}
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write all values of row and all children to f
        for cell in &self.cells {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cell{
    pub column: String,
    pub value: String,
    pub data_type: String,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write all values of cell to f
        write!(f, "{}       ", self.value)?;
        Ok(())
    }
}

// Add a row to a table
pub fn insert(database: &Database, input: String) -> String{
    // Get current time for performance measuring
    let start = std::time::Instant::now();

    // Get table name from input
    let query_table_name = input.split("into").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .trim()
        .to_lowercase();

    // Get table from database
    let matched_table: Option<&Table> = database.tables.iter().find(|&table| table.name.to_lowercase() == query_table_name);

    // If table does not exist, return error
    let matched_table = match matched_table {
        Some(table) => table,
        None => return String::from("Table does not exist"),
    };

    // Get column names from input
    let query_column_names = input.split("into").collect::<Vec<&str>>()[1]
        .split("values")
        .collect::<Vec<&str>>()[0]
        .trim()
        .split("(")
        .collect::<Vec<&str>>()[1]
        .split(")")
        .collect::<Vec<&str>>()[0]
        .split(",")
        .collect::<Vec<&str>>();

    // Get values from input
    let query_values = input.split("values").collect::<Vec<&str>>()[1]
        .trim()
        .split("(")
        .collect::<Vec<&str>>()[1]
        .split(")")
        .collect::<Vec<&str>>()[0]
        .split(",")
        .collect::<Vec<&str>>();

    // Check if number of columns and values match
    if query_column_names.len() != query_values.len() {
        return String::from("Number of columns and values do not match");
    }

    // Check if all column names exist
    for column_name in &query_column_names {
        let matched_column: Option<&Column> = matched_table.columns.iter().find(|&column| column.title.to_lowercase() == column_name.trim().to_lowercase());
        if matched_column.is_none() {
            return String::from("Column does not exist");
        }
    }

    // Create a new row
    let mut new_row = Row{
        cells: Vec::new(),
    };

    // Add cells to row
    for (i, column_name) in query_column_names.iter().enumerate() {
        let matched_column: Option<&Column> = matched_table.columns.iter().find(|&column| column.title.to_lowercase() == column_name.trim().to_lowercase());
        let matched_column = match matched_column {
            Some(column) => column,
            None => return String::from("Column does not exist"),
        };
        let new_cell = Cell{
            column: column_name.trim().to_lowercase(),
            value: query_values[i].trim().to_string(),
            data_type: matched_column.data_type.to_string(),
        };
        new_row.cells.push(new_cell);
    }

    // Add row to table
    let mut new_table = matched_table.clone();
    new_table.rows.push(new_row);

    // matched_table.rows.push(new_row);

    // // Replace table in database
    let mut new_database = database.clone();
    let mut new_tables = Vec::new();
    for table in &new_database.tables {
        if table.name.to_lowercase() == query_table_name {
            new_tables.push(new_table.clone());
        } else {
            new_tables.push(table.clone());
        }
    }

    // // Save database
    new_database.tables = new_tables;
    save_database(new_database);

    // Get elapsed time
    let elapsed = start.elapsed();
    println!("Elapsed time: {}ms", elapsed.as_millis());


    // Return success message
    return format!("{} row(s) inserted", 1);
}

pub fn select(database: &Database, input: String) -> String {
    // Get current time for performance measuring
    let start = std::time::Instant::now();

    // Get table name from input
    let query_table_name = input.split("from").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .trim()
        .to_lowercase();

    // Get table from database
    let matched_table: Option<&Table> = database
        .tables
        .iter()
        .find(|&table| table.name.to_lowercase() == query_table_name);
    // If table does not exist, return error
    let matched_table = match matched_table {
        Some(table) => table,
        None => return String::from("Table does not exist"),
    };

    // Get column names from input
    let query_column_names = input.split("select").collect::<Vec<&str>>()[1]
        .split("from")
        .collect::<Vec<&str>>()[0]
        .trim()
        .split(",")
        .collect::<Vec<&str>>();

    // Get rows from matched_table
    let rows = &matched_table.rows;

    // Create string to return
    let mut return_string = String::from("{\n");

    // Get WHERE statement from input
    let where_statement = input.split("where").collect::<Vec<&str>>()[1].trim();

    // Get column name from WHERE statement
    let where_column_name = where_statement.split("=").collect::<Vec<&str>>()[0].trim();

    // Get value from WHERE statement
    let query_where_value = where_statement.split("=").collect::<Vec<&str>>()[1].trim().replace("\"", "").replace("\'", "");

    // Get column from table
    let matched_column: Option<&Column> = matched_table
        .columns
        .iter()
        .find(|&column| column.title.to_lowercase() == where_column_name);
    // If column does not exist, return error
    let _ = match matched_column {
        Some(column) => column,
        None => return String::from("Column does not exist"),
    };

    // Loop through rows
    for row in rows {
        // Get value from row by iterating through it's cells and finding the cell with the same column name as the where column name
        let value = row.cells.iter().find(|&cell| cell.column.to_lowercase() == where_column_name).unwrap().value.to_lowercase().clone();

        // If value is not equal to where value, continue
        if value != query_where_value {
            continue;
        } else {
            // Loop through column names
            for column_name in &query_column_names {
                // Get column from table
                match column_name.trim() {
                    "*" => {
                        // If column name is *, add all columns to return string
                        for column in &matched_table.columns {
                            // Get value from row
                            let value = row.cells.iter().find(|&cell| cell.column.to_lowercase() == column.title.to_lowercase()).unwrap().value.clone();


                            // Add value to return string
                            return_string
                                .push_str(&format!(r##""{}": "{}","##, column.title.to_lowercase(), value));
                        }
                        break;
                    }
                    _ => {
                        // If column name is not *, add column to return string
                        let column = matched_table
                            .columns
                            .iter()
                            .find(|&column| column.title.to_lowercase() == column_name.trim());
                        let column = match column {
                            Some(column) => column,
                            None => return String::from("Column does not exist"),
                        };
                        // Get value from row
                        let value = row.cells.iter().find(|&cell| cell.column.to_lowercase() == column.title.to_lowercase()).unwrap().value.clone();

                        // Add value to return string
                        return_string.push_str(&format!(r##""{}": "{}","##, column_name.to_lowercase(), value));
                    }
                }
            }

            // Remove last comma from return string
            return_string.pop();
            // Add new line to return string
            return_string.push_str(&format!("\n"));
        }
        break;
    }
    // If return string is empty, return "No rows found"
    if return_string.is_empty() {
        return_string = String::from("No rows found");
    }

    // Add closing bracket to return string
    return_string.push_str(&format!("}}"));

    // Print time passed
    println!("Time passed: {}ms", start.elapsed().as_millis());

    // Return string
    return_string
}

pub fn load_database() -> Database {
    let database_name = String::from("Database");
    if std::path::Path::new(&format!("{}.json", database_name)).exists() {
        println!("Database file exists");
        // If database file exists, load it
        let database_json =
            std::fs::read_to_string(&format!("{}.json", database_name)).expect("Unable to read file");
        // Convert json string to database struct
        let database: Database = serde_json::from_str(&database_json).unwrap();
        database
    } else {
        println!("Database file does not exist");
        // If database file does not exist, create it
        let database = create_database();
        save_database(database.clone());
        database
    }
}

fn create_database() -> Database {
    println!("Creating database");
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

    let password_column = Column {
        title: String::from("Password"),
        data_type: String::from("String"),
    };

    // Create a list of ten users, where the name is "User " + i and the age is a random i32 between 13 and 78
    let mut rng = rand::thread_rng();
    let mut rows = Vec::new();
    for i in 0..1000 {
        // Generate new age as Integer between 13 and 78
        let new_age: i32 = rng.gen_range(13..78);
        // Convert new age to string
        let new_age = new_age.to_string();
        let row = Row {
            cells: vec![
                Cell {
                    column: String::from("Name"),
                    value: format!("User {}", i),
                    data_type: String::from("String"),
                },
                Cell {
                    column: String::from("Age"),
                    value: new_age,
                    data_type: String::from("Int"),
                },
                Cell {
                    column: String::from("Password"),
                    value: String::from("usnKOv8UPdBr0qJpoLBpsQ=="),
                    data_type: String::from("String"),
                },
            ],
        };
        rows.push(row);
    }

    users_table.rows = rows;
    users_table.columns.push(name_column);
    users_table.columns.push(age_column);
    users_table.columns.push(password_column);
    database.tables.push(users_table);
    // Convert database to serde_json object
    database = serde_json::from_str(&serde_json::to_string(&database).unwrap()).unwrap();
    println!("Database created");
    database
}

fn save_database(database: Database) {
    println!("Saving database");
    // Convert database to json string
    let json_string = serde_json::to_string(&database).unwrap();

    // Save json string to file named (database.name).json
    std::fs::write(format!("{}.json", database.name), json_string).expect("Unable to write file");

    // Print success message
    println!("Database saved");
}
