use crate::user::database::*;
use crate::utils::select_date;
use std::io::{self, Write};


pub fn create_user_menu(db_url: &str) {
    let mut name = String::new();
    let mut password = String::new();

    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name = name.trim().to_string();

    print!("Email: ");
    io::stdout().flush().unwrap();
    let email = crate::utils::check_email();

    let date = select_date();

    print!("Password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    password = password.trim().to_string();

    match post_user(db_url, &name, &email, &password, &UserDate::CreationDate(date)) {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Error creating user: {}", e),
    }
}



pub fn read_user_menu(db_url: &str){

    let mut email = String::new();
    let mut choice = String::new();
    let mut password = String::new();

    print!("1- By id, else all users: ");
    
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    choice = choice.trim().to_string();

    if choice == "1"{

        

        print!("Email: ");
        io::stdout().flush().unwrap();
    
        io::stdin().read_line(&mut email).expect("Failed to read line");
    
        email = email.trim().to_string();


        print!("password: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut password).expect("Failed to read line");
        password = password.trim().to_string();
    
        match get_user_by_email(&db_url, email, password) {
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

pub fn edit_user_menu(db_url: &str){

    let mut email = String::new();
    let mut password = String::new();


    let mut new_name = String::new();
    let mut new_password = String::new();


    let (status_line, user_json) = get_all_users(&db_url);

    println!("Status: {}", status_line);
    println!("Response: {}", user_json);

    print!("Email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).expect("Failed to read line");
    email = email.trim().to_string();

    print!("password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    password = password.trim().to_string();

    print!("New Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_name).expect("Failed to read line");
    new_name = new_name.trim().to_string();

    print!("New password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_password).expect("Failed to read line");
    new_password = new_password.trim().to_string();

    match edit_user_by_email(&db_url, &email, &password, &new_name, &new_password){

        Ok(result)=>{

            println!("Response: {}", result);
        }
        Err(e) => {
            println!("Error creating user: {}", e);
        }
    }
    

    
}

pub fn delete_user_menu(db_url: &str){

    let mut email = String::new();
    let mut password = String::new();

    let (status_line, user_json) = get_all_users(&db_url);

    println!("Status: {}", status_line);
    println!("Response: {}", user_json);

    print!("Email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).expect("Failed to read line");
    email = email.trim().to_string();

    print!("password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    password = password.trim().to_string();



  
    match delete_user_by_email(&db_url, email, password) {

        Ok(result)=>{

            println!("Response: {}", result);
        }
        Err(e) => {
            println!("Error creating user: {}", e);
        }
    } 
}
