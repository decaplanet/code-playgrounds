use std::io::stdin;

fn main() {
    println!("Which year do you want to check? :");
    let mut year_input = String::new();
    stdin().read_line(&mut year_input).unwrap();

    let year: u32 = year_input.trim().parse().unwrap();

    if year % 4 == 0 {
        if year % 100 != 0 && year % 400 != 0 {
            println!("Leap year.");
        } else if year % 100 == 0 {
            println!("Not leap year");
        } else {
            println!("Leap year.");
        }
    } else {
        println!("Not leap year.")
    }
}
