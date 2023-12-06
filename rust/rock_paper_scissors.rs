use rand::Rng;
use std::io::stdin;

const RESULTS: [[&str; 3]; 3] = [
    ["It's a draw", "You lose", "You win"],
    ["You win", "It's a draw", "You lose"],
    ["You lose", "You win", "It's a draw"],
];

fn to_ascii_art<'a>(input: usize) -> &'a str {
    match input {
        0 => {
            r#"
_______
---'   ____)
    (_____)
    (_____)
    (____)
---.__(___)
"#
        }
        1 => {
            r#"
_______
---'   ____)____
          ______)
          _______)
         _______)
---.__________)
"#
        }
        2 => {
            r#"
    _______
    ---'   ____)____
                ______)
            __________)
            (____)
    ---.__(___)
"#
        }
        _ => {
            panic!("Invalid Input!");
        }
    }
}

fn rock_paper_scissors() {
    println!("What do you choose? Type 0 for Rock, 1 for Paper or 2 for Scissors. :");

    let mut user_selection = String::new();
    stdin().read_line(&mut user_selection).unwrap();
    let user_selection: usize = user_selection.trim().parse().unwrap();

    let computer_selection: usize = rand::thread_rng().gen_range(0..=2);

    if user_selection < 3 {
        println!("{}", to_ascii_art(user_selection));
        println!("Your computer chose:\n{}", to_ascii_art(computer_selection));
        let result = RESULTS[user_selection][computer_selection];
        println!("{}", result);

        if result == "It's a draw" {
            rock_paper_scissors();
        }
    } else {
        println!("Your input is not valid. Please enter again.");
        rock_paper_scissors();
    }
}

fn main() {
    rock_paper_scissors();
}
