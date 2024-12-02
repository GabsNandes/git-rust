use std::env;
use std::io::{self, Write};

pub mod user;
pub mod utils;

use crate::user::{menu::*, database::set_database};

use crate::user::model::UserDate;

fn main() {
    // Load environment variables
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Error: DATABASE_URL environment variable is not set.");
            return;
        }
    };

    // Set up the database
    if let Err(e) = set_database(&db_url) {
        eprintln!("Error setting up the database: {}", e);
        return;
    }

    loop {
        println!(
            "Choose an option:\n\
             1 - Create a new user\n\
             2 - Read user\n\
             3 - Edit user\n\
             4 - Delete user\n\
             Any other key - Exit"
        );

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => create_user_menu(&db_url),
            "2" => read_user_menu(&db_url),
            "3" => edit_user_menu(&db_url),
            "4" => delete_user_menu(&db_url),
            _ => {
                println!("Exiting...");
                break;
            }
        }
    }
}
