use std::io::{self, Write};

fn main() {
    let mut name_input = String::new();
    print!("\nWhat is your name : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name_input)
        .expect("Failed to read input");

    let mut is_loop = true;
    let mut agreement_input = String::new();

    while is_loop {
        print!("\nCan I spell your name?");
        io::stdout().flush().unwrap();
        print!("\nEnter 'y' or 'n'. : ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut agreement_input)
            .expect("Failed to read input");

        agreement_input = agreement_input.trim().to_string();

        if !agreement_input.is_empty() {
            is_loop = false;
        }
    }

    if agreement_input == "y" {
        for item in name_input.chars() {
            println!("{}", item);
        }
        println!("Here is your name spell, {}!", name_input.trim());
    } else {
        println!("Exiting the program...")
    }
}
