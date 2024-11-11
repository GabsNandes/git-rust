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
    let mut email = String::new();


    print!("name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    
    print!("Email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).expect("Failed to read line");


    
    post_user(&db_url, &name, &email);
}

fn read_user_menu(db_url: &str){

    let mut num = String::new();
    

    print!("ID: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");

    get_user_by_id(&db_url, num);
    
}

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
            _ => {
                println!("Saindo...");
                break;
            },
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &db_url);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
    
}

// Handle client requests
fn handle_client(mut stream: TcpStream, db_url: &str) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
            println!("{}", request);
            let (status_line, content) = match &*request {

                
                r if r.starts_with("POST /users") => handle_post_request(r, db_url),
                r if r.starts_with("GET /users/") => handle_get_request(r, db_url),
                r if r.starts_with("GET /users") => handle_get_all_request(r, db_url),
                r if r.starts_with("PUT /users/") => handle_put_request(r, db_url),
                r if r.starts_with("DELETE /users/") => handle_delete_request(r, db_url),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

// Handle POST request
fn handle_post_request(request: &str, db_url: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(db_url, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO users (name, email) VALUES ($1, $2)",
                    &[&user.name, &user.email],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

// Handle GET request
fn handle_get_request(request: &str, db_url: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(db_url, NoTls)) {
        (Ok(id), Ok(mut client)) => match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
            Ok(row) => {
                let user = User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                };

                (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
            }
            _ => (NOT_FOUND.to_string(), "User not found".to_string()),
        },
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

// Handle GET all request
fn handle_get_all_request(_request: &str, db_url: &str) -> (String, String) {
    match Client::connect(db_url, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT id, name, email FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

// Handle PUT request
fn handle_put_request(request: &str, db_url: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        get_user_request_body(&request),
        Client::connect(db_url, NoTls),
    ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &id],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

// Handle DELETE request
fn handle_delete_request(request: &str, db_url: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(db_url, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();

            // If rows affected is 0, user not found
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
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
