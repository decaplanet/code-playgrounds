use std::{env, fs};
pub mod settings;

pub const LINE_WORD_LIMIT: usize = 8;
pub const LINE_LETTER_LIMIT: usize = 70;

fn main() {
    let mut args = env::args();

    let file_path = args.nth(1).unwrap_or_else(|| {
        eprintln!("Error: The program needs an absolute path as an argument to perform.");
        std::process::exit(1);
    });

    if args.next().is_some() {
        eprintln!("Error: The program can only take one argument.");
        std::process::exit(1);
    }

    let contents = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!("Keep in mind that an absolute path must be provided.");
            return;
        }
    };

    let divided_contents = contents
        .split(' ')
        .flat_map(|part| part.split_terminator('\n'))
        .collect::<Vec<_>>();

    let mut line_contents: Vec<String> = Vec::new();
    let mut one_line = String::new();

    for (index, item) in divided_contents.iter().enumerate() {
        let line_word_length = one_line.split_terminator(' ').collect::<Vec<&str>>().len();

        if item.is_empty() {
            one_line.push('\n');
            line_contents.push(one_line.clone());
            one_line.clear();
        } else if within_limit(line_word_length, one_line.len()) {
            one_line.push_str(item);
            if within_limit(line_word_length, one_line.len()) {
                one_line.push(' ');
            }

            if index >= divided_contents.len() - 1 {
                line_contents.push(one_line.clone());
                one_line.clear();
            }
        } else {
            line_contents.push(one_line.clone());
            one_line.clear();
        }
    }

    let formatted_contents = line_contents.join("\n");

    println!("\n✅ FORMATTED VERSION :");
    println!("{}", formatted_contents);
}

fn within_limit(line_word_length: usize, one_line_length: usize) -> bool {
    line_word_length <= LINE_WORD_LIMIT || one_line_length <= LINE_LETTER_LIMIT
}
