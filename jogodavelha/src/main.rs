use std::collections::HashMap;
use std::io::{self, Write};

fn print_table(table: &HashMap<String, String>) {
    let mut count = 0;
    println!();
    for i in 1..=9 {
        if let Some(value) = table.get(&i.to_string()) {
            print!("| {} ", value);
        } else {
            print!("|   "); // Display empty space for unoccupied cells
        }
        count += 1;
        if count == 3 {
            println!("|");
            count = 0;
        }
    }
    println!(); // Extra line after the table for better readability
}

fn build_game(table: &mut HashMap<String, String>) {
    for i in 1..=9 {
        table.insert(i.to_string(), "".to_string()); // Initialize with empty strings
    }
}


fn game(table: &mut HashMap<String, String>, player: &str) {
    let mut key = String::new();
    
    loop {
        print!("{player}: escolha a casa (1-9): ");
        print_table(table);
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut key).expect("Failed to read line");
        let key_trimmed = key.trim().to_string();

        if let Some(value) = table.get(&key_trimmed) {
            if value == "x" || value == "o" {
                println!("{player}: já existe {value} nessa posição, insira uma posição válida.");
            } else {
                // Place the player's mark
                if player == "playerX" {
                    table.insert(key_trimmed.clone(), "x".to_string());
                } else {
                    table.insert(key_trimmed.clone(), "o".to_string());
                }
                break; // Valid move, exit the loop
            }
        } else {
            println!("{player}: posição inválida, insira uma posição válida (1-9).");
        }
        key.clear(); // Clear the key for the next input attempt
    }

    print_table(table);
}

fn check_winner(table: &HashMap<String, String>) -> Option<String> {
    // Define winning combinations
    let win_conditions = [
        // Rows
        ["1", "2", "3"],
        ["4", "5", "6"],
        ["7", "8", "9"],
        // Columns
        ["1", "4", "7"],
        ["2", "5", "8"],
        ["3", "6", "9"],
        // Diagonals
        ["1", "5", "9"],
        ["3", "5", "7"],
    ];

    
    for condition in win_conditions.iter() {
        
        let first = table.get(condition[0]);
        let second = table.get(condition[1]);
        let third = table.get(condition[2]);

        
        if first == Some(&"x".to_string()) && second == Some(&"x".to_string()) && third == Some(&"x".to_string()) {
            return Some("X".to_string()); 
        } else if first == Some(&"o".to_string()) && second == Some(&"o".to_string()) && third == Some(&"o".to_string()) {
            return Some("O".to_string()); 
        }
    }
    None // No winner found
}


fn main() {
    let mut table: HashMap<String, String> = HashMap::new();
    let playerx = "playerX";
    let playero = "playerO";
    let mut plays = 1;

    build_game(&mut table);

    loop {
        if plays % 2 == 0 {
            game(&mut table, playero);
        } else {
            game(&mut table, playerx);
        }

        if let Some(winner) = check_winner(&mut table) {
            println!("{} venceu!", winner);
            break;
        }

        // Check for draw (if all positions are filled)
        if plays >= 9 {
            println!("Empate!");
            break;
        }

        plays += 1; // Increment plays to switch to the next player
    }
    print_table(&table);
}
