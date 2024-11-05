use std::fs::File;
use std::io::{self, Write};
use std::{error::Error, process};
use serde::{Deserialize, Serialize};
use csv::Writer;



// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {
    id: i32,
    name: String,
    email: String,
    dateofbirth: String
}

fn id_generator(users: &[User]) -> i32{

    if users.is_empty() {
        return 1;
    }
    
    let max_id = users.iter().map(|user| user.id).max().unwrap();
    
    max_id + 1



}

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

        input
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

    let mut name = String::new();
    let mut dateofbirth = String::new();



    let num: i32 = id_generator(users);
    println!("ID: {}", num);

    print!("name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name).expect("Failed to read line");

    
    let email = check_email();

    print!("Date of birth: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut dateofbirth).expect("Failed to read line");


    let new_user = create_user(num, name, email, dateofbirth);
    users.push(new_user)
}

fn create_user(id:i32, name:String, email:String, dateofbirth: String) -> User{

    User{

        id:id,
        name:name,
        email:email,
        dateofbirth:dateofbirth

    }


}

fn read_user_menu(users: &[User]){

    let mut num = String::new();
    

    print!("ID: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");

    if let Some(user) = read_user_by_id(&users, num) {
        println!("Found user with ID {}: {}, {}, {}", user.id, user.name, user.email, user.dateofbirth);
    }
}


fn read_user_by_id(users: &[User], target_id: i32) -> Option<&User> {
    
    users.iter().find(|&user| user.id == target_id)

}

fn update_user_menu(users: &mut Vec<User>){

    let mut num = String::new();
    let mut name = String::new();
    let mut dateofbirth = String::new();

    print!("ID: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");

    print!("name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name).expect("Failed to read line");

    let email = check_email();

    print!("Date: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut dateofbirth).expect("Failed to read line");
    

    update_user(users, num, &name, &email, &dateofbirth);


}

fn update_user(users: &mut Vec<User>, target_id: i32, new_name: &str, new_email: &str, new_dob: &str){

    if let Some(user) = users.iter_mut().find(|user| user.id == target_id) {
        user.name = new_name.to_string();
        user.email = new_email.to_string();
        user.dateofbirth = new_dob.to_string();
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

    

    let file_path = String::from("users.csv");


    let mut users: Vec<User> = match load_users(&file_path) {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_read_user_test() {
        let num = 69;
        let name = "Gabriel";
        let email = "gb@jhan.com";
        let dateofbirth="23/05?2001";
        let value = create_user(num, name.to_string(), email.to_string(), dateofbirth.to_string());
        
        

        if let Some(user) = read_user_by_id(&[value], num) {
            assert_eq!(user.name, name);
            assert_eq!(user.email, email);
            assert_eq!(user.dateofbirth, dateofbirth);
        }


    }

    #[test]
    fn update_read_user_test() {

        let num = 2;
        
        
        let new_name = "Gabriel Maga";
        let new_email = "gabbb@jhan.com";
        let new_dateofbirth="23/05/2001";

        let mut users: Vec<User> = Vec::new();

        let name = "Gabriel";
        let email = "gb@jhan.com";
        let dateofbirth="23/05?2001";
        let value = create_user(num, name.to_string(), email.to_string(), dateofbirth.to_string());
        users.push(value.clone());

        update_user(&mut users, num, &new_name, &new_email, &new_dateofbirth);
        

        if let Some(user) = read_user_by_id(&users, num) {
            assert_eq!(user.name, new_name);
            assert_eq!(user.email, new_email);
            assert_eq!(user.dateofbirth, new_dateofbirth);
        }


    }

    #[test]
    
    fn delete_user_test() {
        let num = 69;
        let name = "Gabriel";
        let email = "gb@jhan.com";
        let dateofbirth="23/05?2001";

        let mut users: Vec<User> = Vec::new();


        let value = create_user(num, name.to_string(), email.to_string(), dateofbirth.to_string());
        
        users.push(value.clone());

        let deleted = delete_user(&mut users, num);

        if let Some(user) = read_user_by_id(&[value], num) {
            assert_eq!(user.name, name);
            assert_eq!(user.email, email);
            assert_eq!(user.dateofbirth, dateofbirth);
        }

        assert_eq!(deleted, true);







    }

}
