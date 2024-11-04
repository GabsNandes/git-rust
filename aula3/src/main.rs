fn main() {
    

    let mut trimmed_input = "";

    // Trim '@' and '!' characters from both ends
    if input.ends_with(".com") {
        trimmed_input = input.strip_suffix(".com").unwrap();
    } else{
        trimmed_input = input;
    }

    println!("Original: {}", input);
    println!("Trimmed: {}", trimmed_input);

    if &input == &trimmed_input {
        println!("invalid");
    }
}
