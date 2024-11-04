use std::fs::File;
use std::io::{self, Write};
use std::{error::Error, process};
use serde::{Deserialize, Serialize};
use csv::Writer;
use std::env;



// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Function to save users to a CSV file
fn save_users_to_csv(users: &[User], file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    for user in users {
        wtr.serialize(user)?;
    }
    wtr.flush()?;
    Ok(())
}

fn load_users(file_path: &str) -> Result<Vec<User>, Box<dyn Error>>{

    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut users = Vec::new();
    for result in rdr.deserialize() {
        let user: User = result?;
        users.push(user);
    }
    Ok(users)


}


fn create_user_menu(users: &mut Vec<User>){

    let mut num = String::new();
    let mut name = String::new();
    let mut email = String::new();

    print!("ID: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");

    print!("name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name).expect("Failed to read line");


    print!("email: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut email).expect("Failed to read line");


    let new_user = create_user(num, name, email);
    users.push(new_user)
}

fn create_user(id:i32, name:String, email:String) -> User{

    User{

        id:id,
        name:name,
        email:email

    }


}

fn read_user_menu(users: &[User]){

    let mut num = String::new();
    

    print!("ID: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");

    if let Some(user) = read_user_by_id(&users, num) {
        println!("Found user with ID {}: {}, {}", user.id, user.name, user.email);
    }
}


fn read_user_by_id(users: &[User], target_id: i32) -> Option<&User> {
    
    users.iter().find(|&user| user.id == target_id)

}

fn update_user_menu(users: &mut Vec<User>){

    let mut num = String::new();
    let mut name = String::new();
    let mut email = String::new();

    print!("ID: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");

    print!("name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name).expect("Failed to read line");


    print!("email: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut email).expect("Failed to read line");

    update_user(users, num, &name, &email);


}

fn update_user(users: &mut Vec<User>, target_id: i32, new_name: &str, new_email: &str){

    if let Some(user) = users.iter_mut().find(|user| user.id == target_id) {
        user.name = new_name.to_string();
        user.email = new_email.to_string();
    }

}

fn delete_user_menu(users: &mut Vec<User>){

    let mut num = String::new();
    

    print!("ID: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");

    delete_user(users, num);
    
}

fn delete_user(users: &mut Vec<User>, target_id: i32) -> bool {
   
    if let Some(index) = users.iter().position(|user| user.id == target_id) {
        users.remove(index);
        true
    } else {
        false
    }
}


fn main(){

    // Retrieve the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure that a file path is provided
    if args.len() < 2 {
        eprintln!("Usage: cargo run <file_path>");
        process::exit(1);
    }

    let file_path = &args[1];

    let mut users: Vec<User> = match load_users(file_path) {
        Ok(users) => users,
        Err(err) => {
            println!("Error loading users from CSV: {}", err);
            process::exit(1);
        }
    };
    
    loop {
        let mut num = String::new();

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut num).expect("Failed to read line");

        let num: i32 = match num.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Por favor, insira um número válido.");
                continue;
            }
        };

        match num {
            1 => create_user_menu(&mut users),
            2 => read_user_menu(&users),
            3 => update_user_menu(&mut users),
            4 => delete_user_menu(&mut users),
            5 => {
                if let Err(err) = save_users_to_csv(&users, "users.csv") {
                    println!("Error saving users to CSV: {}", err);
                } else {
                    println!("Users saved successfully to users.csv.");
                }
                break;
            },
            _ => break,
        }
    }

    
    
}
