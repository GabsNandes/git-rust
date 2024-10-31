use std::env;
use std::io::{self, Write};


struct User {
    id: i32,
    name: String,
    email: String,
}


fn create_user_menu(users: &mut Vec<User>){

    let mut num = String::new();
    let mut name = String::new();
    let mut email = String::new();

    print!("ID: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("REASON");;

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

    let num: i32 = num.trim().parse().expect("REASON");;

    let Some(user) = read_user_by_id(&users, num) else { todo!() };;
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

    let num: i32 = num.trim().parse().expect("REASON");;

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

    let num: i32 = num.trim().parse().expect("REASON");;

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

    let mut users: Vec<User> = Vec::new();
    
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
            _ => break,
        }
    }
    
    

    

    

}
