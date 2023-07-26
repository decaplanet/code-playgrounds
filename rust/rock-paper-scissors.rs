use rand::Rng;
use std::{io::stdin, vec};

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

fn rock_paper_scissors(results: &Vec<Vec<&str>>) {
    println!("What do you choose? Type 0 for Rock, 1 for Paper or 2 for Scissors. :");

    let mut user_selection = String::new();
    stdin().read_line(&mut user_selection).unwrap();
    let user_selection: usize = user_selection.trim().parse().unwrap();

    let computer_selection: usize = rand::thread_rng().gen_range(0..=2);

    if user_selection < 3 {
        println!("{}", to_ascii_art(user_selection));
        println!("Your computer chose:\n{}", to_ascii_art(computer_selection));
        let result = results[user_selection][computer_selection];
        println!("{}", result);

        if result == "It's a draw" {
            rock_paper_scissors(results);
        }
    } else {
        println!("Your input is not valid. Please enter again.");
        rock_paper_scissors(results);
    }
}

fn main() {
    let results: Vec<Vec<&str>> = vec![
        vec!["It's a draw", "You lose", "You win"],
        vec!["You win", "It's a draw", "You lose"],
        vec!["You lose", "You win", "It's a draw"],
    ];

    rock_paper_scissors(&results)
}
