use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::env;
use serde_json::json;
use std::error::Error;

#[macro_use]
extern crate serde_derive;

// Model: User struct with id, name, email
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// Constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";


//Menus:

fn create_user_menu(db_url: &str){

    let mut name = String::new();


    print!("name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name = name.trim().to_string();
    
    let email = check_email();


    
    match post_user(&db_url, &name, &email){

        Ok(result)=>{

            println!("Response: {}", result);
        }
        Err(e) => {
            println!("Error creating user: {}", e);
        }
    }
}

fn read_user_menu(db_url: &str){

    let mut num = String::new();
    let mut choice = String::new();

    print!("1- By id, else all users: ");
    
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut num).expect("Failed to read line");

    

    if choice == "1"{

        

        print!("ID: ");
        io::stdout().flush().unwrap();
    
        io::stdin().read_line(&mut num).expect("Failed to read line");
    
        let num: i32 = num.trim().parse().expect("REASON");
    
        match get_user_by_id(&db_url, num) {
            Ok(user_json) => {
                println!("User found: {}", user_json);
            }
            Err(e) => {
                println!("Error fetching user: {}", e);
            }
        } 

    }else{

        let (status_line, user_json) = get_all_users(&db_url);

        println!("Status: {}", status_line);
        println!("Response: {}", user_json);
        

    }
    

    
}

fn edit_user_menu(db_url: &str){

    let mut num = String::new();
    let mut choice = String::new();
    let mut name = String::new();

    let (status_line, user_json) = get_all_users(&db_url);

    println!("Status: {}", status_line);
    println!("Response: {}", user_json);

    print!("Id: ");
    
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");

    print!("name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name.trim();

    
    let email = check_email();

    match edit_user_by_id(&db_url, num, &name, &email){

        Ok(result)=>{

            println!("Response: {}", result);
        }
        Err(e) => {
            println!("Error creating user: {}", e);
        }
    }
    

    
}

fn delete_user_menu(db_url: &str){

    let mut num = String::new();

    let (status_line, user_json) = get_all_users(&db_url);

    println!("Status: {}", status_line);
    println!("Response: {}", user_json);

    print!("Id: ");
    
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut num).expect("Failed to read line");

    
    let num: i32 = num.trim().parse().expect("REASON");

  
    match delete_user_by_id(&db_url, num) {

        Ok(result)=>{

            println!("Response: {}", result);
        }
        Err(e) => {
            println!("Error creating user: {}", e);
        }
    } 

   

    
}


//CLI functions

fn post_user(db_url: &str, name: &str, email:&str) -> Result<String, Box<dyn Error>> {
    // Connect to the database
    let mut client = Client::connect(db_url, NoTls)?;

    // Insert the new user into the database
    let result = client.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&name, &email],
    );

    // Check if the insertion was successful
    match result {
        Ok(_) => Ok("User created successfully".to_string()),
        Err(e) => Err(Box::new(e)), // Return error if the insert fails
    }
}

fn get_user_by_id(db_url: &str, user_id: i32) -> Result<String, Box<dyn Error>> {
    
    let mut client = Client::connect(db_url, NoTls)?;

    // Query for the user
    let result = client.query_opt("SELECT id, name, email FROM users WHERE id = $1", &[&user_id]);

    // Check if the user exists
    match result {
        Ok(Some(row)) => {
            // Create a User struct to hold the result
            let user = json!({
                "id": row.get::<_, i32>(0),
                "name": row.get::<_, String>(1),
                "email": row.get::<_, String>(2),
            });

            // Return the user as a JSON string
            Ok(user.to_string())
        }
        Ok(None) => Ok("User not found".to_string()), // Return a message if user doesn't exist
        Err(e) => Err(Box::new(e)), // Return error if query fails
    }


}

fn get_all_users(db_url: &str) -> (String, String) {

    let mut client = match Client::connect(db_url, NoTls) {
        Ok(client) => client,
        Err(e) => return (INTERNAL_ERROR.to_string(), format!("Error connecting to the database: {}", e)),
    };

    let mut users = Vec::new();

    match client.query("SELECT id, name, email FROM users", &[]) {
        Ok(rows) => {
            for row in rows {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }
            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        Err(e) => {
            (INTERNAL_ERROR.to_string(), format!("Error querying the database: {}", e))
        }
    }
}

fn edit_user_by_id(db_url: &str, user_id: i32, name: &str, email:&str) -> Result<String, Box<dyn Error>> {


    let mut client = Client::connect(db_url, NoTls)?;
    
    
    let result = client.execute(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&name, &email, &user_id],
    );

    
    match result {
        Ok(_) => Ok("User edited successfully".to_string()),
        Err(e) => Err(Box::new(e)), 
    }
    }

fn delete_user_by_id(db_url: &str, user_id: i32) -> Result<String, Box<dyn Error>> {

    let mut client = Client::connect(db_url, NoTls)?;


    let result = client.execute(
        "DELETE FROM users WHERE id = $1", &[&user_id],
    );


    match result {
        Ok(_) => Ok("User deleted successfully".to_string()),
        Err(e) => Err(Box::new(e)),
    }
}





//Main:

fn main() {
    // Retrieve DATABASE_URL at runtime
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Error: DATABASE_URL environment variable is not set.");
            return;
        }
    };

    // Set up the database
    if let Err(_) = set_database(&db_url) {
        println!("Error setting up the database");
        return;
    }

    // Start the server and print port
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on port 8080");

    loop {

        print!("Escolha uma opção: ");

        
        io::stdout().flush().unwrap(); // Make sure the prompt is displayed

        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap(); // Read input from user

    
        let num = num.trim(); 
    
        match num {
            "1" => create_user_menu(&db_url),
            "2" => read_user_menu(&db_url),
            "3" => edit_user_menu(&db_url),
            "4" => delete_user_menu(&db_url),
            _ => {
                println!("Saindo...");
                break;
            },
        }
    }
    
}

// Database setup
fn set_database(db_url: &str) -> Result<(), PostgresError> {
    let mut client = Client::connect(db_url, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    "
    )?;
    Ok(())
}

// Get id from request URL
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// Deserialize user from request body without id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

//utils

#[allow(unused_assignments)]
fn check_email() -> String{


    let mut input = String::new(); 
    let mut trimmed_input = String::new(); 

    loop{
        print!("email: ");
        
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        trimmed_input = input.trim().to_string();

        if trimmed_input.ends_with(".com") {
            trimmed_input = trimmed_input.strip_suffix(".com").unwrap().to_string();
        } 

    
        if &input.trim() == &trimmed_input {
            
            println!("invalid email, try again");

        }else{
            
            if input.contains("@") {
                break
            }else{
                println!("Invalid Email");
                continue
            }
        }
    }

    input.trim().to_string()
    }

    
