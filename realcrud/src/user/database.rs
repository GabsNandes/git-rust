use postgres::{Client, NoTls, Error as PostgresError};
use crate::user::model::{User, UserDate};
use bcrypt::{hash, verify};
use serde_json::json;
use crate::utils::verify_password;
use crate::utils::hash_password;
use crate::constants::{INTERNAL_ERROR, OK_RESPONSE};

// Database setup
fn set_database(db_url: &str) -> Result<(), PostgresError> {
    let mut client = Client::connect(db_url, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
            password VARCHAR NOT NULL,
            date VARCHAR NOT NULL
        )
    "
    )?;
    Ok(())
}

pub fn post_user(
    db_url: &str,
    name: &str,
    email: &str,
    password: &str,
    date: &UserDate,
) -> Result<String, Box<dyn std::error::Error>> {
    let hashed_password = hash(password, 4)?;
    let mut client = Client::connect(db_url, NoTls)?;
    client.execute(
        "INSERT INTO users (name, email, password, date) VALUES ($1, $2, $3, $4)",
        &[&name, &email, &hashed_password, &date.to_db_string()],
    )?;
    Ok("User created successfully".to_string())
}



pub fn get_user_by_email(
    db_url: &str,
    email: String,
    password: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = Client::connect(db_url, NoTls)?;

    // Query for the user
    let result = client.query_opt(
        "SELECT id, name, email, password, date FROM users WHERE email = $1",
        &[&email],
    );

    match result {
        Ok(Some(row)) => {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            let email: String = row.get(2);
            let hashed_password: String = row.get(3);
            let date: String = row.get(4);

            // Verify the provided password against the hashed password
            if verify_password(&password, &hashed_password)? {
                let user = json!({
                    "id": id,
                    "name": name,
                    "email": email,
                    "date": date,
                });
                Ok(user.to_string())
            } else {
                Ok("Invalid password".to_string())
            }
        }
        Ok(None) => Ok("User not found".to_string()), // Return a message if the user doesn't exist
        Err(e) => Err(Box::new(e)), // Return error if query fails
    }
}


pub fn get_all_users(db_url: &str) -> (String, String) {

    let mut client = match Client::connect(db_url, NoTls) {
        Ok(client) => client,
        Err(e) => return (INTERNAL_ERROR.to_string(), format!("Error connecting to the database: {}", e)),
    };

    let mut users = Vec::new();

    match client.query("SELECT id, name, email, password, date FROM users", &[]) {
        Ok(rows) => {
            for row in rows {
                
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: "*********".to_string(),
                    password: "*********".to_string(),
                    date: match UserDate::from_db_string(&row.get::<_, String>(4)) {
                        Ok(date) => date,
                        Err(e) => {
                            return (
                                INTERNAL_ERROR.to_string(),
                                format!("Error parsing date field: {}", e),
                            )
                        }
                    },

                    
                    
                });
            }
        
            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        
        }
        Err(e) => {
            (INTERNAL_ERROR.to_string(), format!("Error querying the database: {}", e))
        }
    }
}

pub fn edit_user_by_email(
    db_url: &str,
    email: &str,
    current_password: &str,
    new_name: &str,
    new_password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = Client::connect(db_url, NoTls)?;

    // Fetch the current hashed password
    let result = client.query_opt(
        "SELECT password FROM users WHERE email = $1",
        &[&email],
    );

    match result {
        Ok(Some(row)) => {
            let hashed_password: String = row.get(0);

            // Verify the current password
            if verify_password(&current_password, &hashed_password)? {
                // Hash the new password
                let new_hashed_password = hash_password(new_password)?;

                // Update the user details
                client.execute(
                    "UPDATE users SET name = $2, password = $3 WHERE email = $1",
                    &[&email, &new_name, &new_hashed_password],
                )?;
                Ok("User updated successfully".to_string())
            } else {
                Ok("Invalid current password".to_string())
            }
        }
        Ok(None) => Ok("User not found".to_string()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn delete_user_by_email(
    db_url: &str,
    email: String,
    password: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = Client::connect(db_url, NoTls)?;

    // Fetch the user's hashed password
    let result = client.query_opt(
        "SELECT password FROM users WHERE email = $1",
        &[&email],
    );

    match result {
        Ok(Some(row)) => {
            let hashed_password: String = row.get(0);

            // Verify the provided password against the hashed password
            if verify_password(&password, &hashed_password)? {
                // If the password is correct, delete the user
                client.execute(
                    "DELETE FROM users WHERE email = $1",
                    &[&email],
                )?;
                Ok("User deleted successfully".to_string())
            } else {
                Ok("Invalid password".to_string())
            }
        }
        Ok(None) => Ok("User not found".to_string()),
        Err(e) => Err(Box::new(e)), // Handle query errors
    }
}

