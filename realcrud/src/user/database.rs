use postgres::{Client, NoTls, Error as PostgresError};
use crate::user::model::{User, UserDate};
use bcrypt::{hash, verify};
use serde_json::json;
use crate::utils::verify_password;
use crate::utils::hash_password;


pub const INTERNAL_ERROR: &str = "500 Internal Server Error";
pub const OK_RESPONSE: &str = "200 OK";

// Database setup
pub fn set_database(db_url: &str) -> Result<(), PostgresError> {
    let mut client = Client::connect(db_url, NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
            password VARCHAR NOT NULL,
            date VARCHAR NOT NULL
        );
        ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS post (
            user_id INTEGER PRIMARY KEY,
            post TEXT NOT NULL,
            name VARCHAR NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id),
        );
        ",
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
    // Define constants
    const INTERNAL_ERROR: &str = "500";
    const OK_RESPONSE: &str = "200";

    // Connect to the database
    let mut client = match Client::connect(db_url, NoTls) {
        Ok(client) => client,
        Err(e) => {
            return (
                INTERNAL_ERROR.to_string(),
                format!("Error connecting to the database: {}", e),
            )
        }
    };

    let mut users = Vec::new();

    // Query the users table
    match client.query("SELECT id, name, email, password, date FROM users", &[]) {
        Ok(rows) => {
            for row in rows {
                let user = User {
                    id: row.get(0),
                    name: row.get(1),
                    email: "*********".to_string(), // Mask email
                    password: "*********".to_string(), // Mask password
                    date: {
                        let db_date: String = row.get(4); // Retrieve as a string
                        UserDate::from_db_string(&db_date) // Convert to UserDate
                    },
                };
                users.push(user);
            }

            // Return success response
            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&users).unwrap_or_else(|_| "[]".to_string()),
            )
        }
        Err(e) => (
            INTERNAL_ERROR.to_string(),
            format!("Error querying the database: {}", e),
        ),
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
// Define the make_post function
pub fn make_post(db_url: &str, email: &str, current_password: &str, post_content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Connect to the database
    let mut client = Client::connect(db_url, NoTls)?;

    // Check if the user exists and retrieve their hashed password and user id
    let result = client.query_opt(
        "SELECT password, id, name FROM users WHERE email = $1",
        &[&email],
    );

    // Handle the case where the user is not found
    match result {
        Ok(Some(row)) => {
            let hashed_password: String = row.get(0);
            let user_id: i32 = row.get(1);
            let poster_name: String = row.get(2);

            // Verify the password
            if verify(current_password, &hashed_password)? {
                // Insert the post into the post table
                client.execute(
                    "INSERT INTO post (user_id, post, name) VALUES ($1, $2, $3)",
                    &[&user_id, &post_content, &poster_name],
                )?;

                Ok(format!("Post created successfully for user id {}", user_id))
            } else {
                // Return an error message if the password is incorrect
                Ok("Incorrect password.".to_string())
            }
        },
        Ok(None) => Ok("User not found".to_string()),
        Err(e) => Err(Box::new(e)), // Handle query errors
    }

}

pub fn get_all_posts(db_url: &str) -> Result<String, PostgresError> {
    // Connect to the database
    let mut client = Client::connect(db_url, NoTls)?;

    // Query to fetch all posts along with user information
    let rows = client.query(
        "
        SELECT users.name, post.post
        FROM post
        JOIN users ON post.user_id = users.id
        ORDER BY post.id DESC
        ", 
        &[]
    )?;

    // Collect posts into a string or JSON format
    let mut posts = Vec::new();
    
    for row in rows {
        let user_name: String = row.get(0);
        let post_content: String = row.get(1);
        
        posts.push(format!(
            "Post by {} : \n{}\n", 
            user_name, post_content
        ));
    }

    // Return the formatted posts as a string
    if posts.is_empty() {
        Ok("No posts available.".to_string())
    } else {
        Ok(posts.join("\n"))
    }
}


pub fn get_all_posts_by_me(db_url: &str, email: &str) -> Result<String, PostgresError> {
    // Connect to the database
    let mut client = Client::connect(db_url, NoTls)?;


    // Query to fetch all posts along with user information
    let rows = client.query(
        "
        SELECT users.name, post.post
        FROM post
        JOIN users ON post.user_id = users.id 
        WHERE users.email = $1
        ORDER BY post.id DESC
        ", 
        &[&email]
    )?;

    // Collect posts into a string or JSON format
    let mut posts = Vec::new();
    
    for row in rows {
        let user_name: String = row.get(0);
        let post_content: String = row.get(1);
        
        posts.push(format!(
            "Post by {} : \n{}\n", 
            user_name, post_content
        ));
    }

    // Return the formatted posts as a string
    if posts.is_empty() {
        Ok("No posts available.".to_string())
    } else {
        Ok(posts.join("\n"))
    }
}

