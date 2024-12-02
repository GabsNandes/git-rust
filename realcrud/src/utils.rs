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

        let months31: Vec<i32> = vec![1, 3, 5, 7, 8, 10, 12]; 
        let months30: Vec<i32> = vec![4, 6, 9, 11];          
        
        print!("Year: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut year).expect("Failed to read line");

        print!("Month: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut month).expect("Failed to read line");

        // Trim and parse inputs
        if let (Ok(month), Ok(year)) = (
            month.trim().parse::<i32>(),
            year.trim().parse::<i32>(),
        ) {
            let final_day = if months31.contains(&month) {
                31
            } else if months30.contains(&month) {
                30
            } else if month == 2 {
                if is_leap_year(year) {
                    29
                } else {
                    28
                }
            } else {
                println!("Invalid month. Please try again.");
                continue;
            };

            print!("Day: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut day).expect("Failed to read line");

            // Parse and validate the day
            if let Ok(day) = day.trim().parse::<i32>() {
                if (1..=final_day).contains(&day) && (1900..=2024).contains(&year) {
                    return format!("{:02}-{:02}-{}", day, month, year);
                } else {
                    println!("Invalid date. Please try again.");
                }
            } else {
                println!("Invalid day. Please enter a numeric value.");
            }
        } else {
            println!("Invalid input. Please enter numeric values for month and year.");
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
