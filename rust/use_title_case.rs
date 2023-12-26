fn main() {
    let input = "the world of technology";
    let output = use_title_case(input);

    println!("{}", output);
}

fn use_title_case(input: &str) -> String {
    let mut title_cased_string = String::new();
    let words: Vec<&str> = input.split(' ').collect();

    for (index, word) in words.iter().enumerate() {
        if word.is_empty() {
            continue;
        }

        if index != 0 && (PREPOSITIONS.contains(word) || ARTICLES.contains(word)) {
            title_cased_string.push_str(word);
        } else {
            let mut word_chars = word.chars();
            let first_letter = {
                let temp_first_letter = word_chars.next().unwrap();

                // Capitalize only when a word is alphabetic.
                if temp_first_letter.is_alphabetic() {
                    temp_first_letter.to_uppercase().next().unwrap()
                } else {
                    temp_first_letter
                }
            };

            let other_letters: String = word_chars.collect();
            title_cased_string.push_str(&format!("{}{}", first_letter, other_letters));
        }

        if index != words.len() - 1 {
            title_cased_string.push(' ');
        }
    }

    title_cased_string
}

const ARTICLES: [&str; 3] = ["a", "an", "the"];
const PREPOSITIONS: [&str; 50] = [
    "about",
    "above",
    "across",
    "after",
    "against",
    "along",
    "amid",
    "among",
    "around",
    "at",
    "before",
    "behind",
    "below",
    "beneath",
    "beside",
    "between",
    "beyond",
    "by",
    "concerning",
    "considering",
    "despite",
    "down",
    "during",
    "except",
    "for",
    "from",
    "in",
    "inside",
    "into",
    "like",
    "near",
    "of",
    "on",
    "onto",
    "out",
    "outside",
    "over",
    "past",
    "regarding",
    "round",
    "since",
    "through",
    "to",
    "toward",
    "under",
    "until",
    "up",
    "with",
    "within",
    "without",
];
