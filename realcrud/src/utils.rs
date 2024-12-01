use bcrypt::{hash, verify};
use std::io::{self, Write};
use regex::Regex;

#[allow(unused_assignments)]
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, 4) // Adjust cost factor as needed
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed)
}

pub fn validate_email(email: &str) -> bool {
    let re = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    re.is_match(email)
}

pub fn check_email() -> String {
    let mut email = String::new();
    loop {
        print!("Email: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut email).expect("Failed to read line");
        email = email.trim().to_string();
        if validate_email(&email) {
            break;
        } else {
            println!("Invalid email. Please try again.");
        }
    }
    email
}

pub fn select_date() -> String {
    loop {
        let mut day = String::new();
        let mut month = String::new();
        let mut year = String::new();

        print!("Day: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut day).expect("Failed to read line");

        print!("Month: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut month).expect("Failed to read line");

        print!("Year: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut year).expect("Failed to read line");

        // Trim and parse input
        if let (Ok(day), Ok(month), Ok(year)) = (
            day.trim().parse::<u32>(),
            month.trim().parse::<u32>(),
            year.trim().parse::<u32>(),
        ) {
            // Validate date ranges
            if (1..=31).contains(&day) && (1..=12).contains(&month) && (1900..=2024).contains(&year) {
                return format!("{:02}-{:02}-{}", day, month, year);
            } else {
                println!("Invalid date. Please try again.");
            }
        } else {
            println!("Invalid input. Please enter numeric values.");
        }
    }
}
