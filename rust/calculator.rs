enum Calculation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

fn main() {
    let value_1 = get_read_line("Enter first number.");
    let mut value_1 = value_1.trim().parse::<f64>().unwrap();

    loop {
        let cal_type = get_read_line("Select an operation.\n+, -, *, /");
        let cal_type = match cal_type.as_str().trim() {
            "+" => Calculation::Addition,
            "-" => Calculation::Subtraction,
            "*" => Calculation::Multiplication,
            "/" => Calculation::Division,
            _ => unreachable!(),
        };

        let value_2 = get_read_line("Enter second number.");
        let value_2 = value_2.trim().parse::<f64>().unwrap();

        value_1 = calculate(value_1, value_2, cal_type);

        println!("Result: {}", value_1);

        let mut is_continuing = String::new();
        println!("Do you want to do another operation? y/n");
        std::io::stdin().read_line(&mut is_continuing).unwrap();

        if is_continuing.contains('n') {
            break;
        }
    }
}

fn get_read_line(message: &str) -> String {
    let mut value = String::new();
    println!("{}", message);
    std::io::stdin().read_line(&mut value).unwrap();

    value
}

fn calculate(value_1: f64, value_2: f64, cal_type: Calculation) -> f64 {
    match cal_type {
        Calculation::Addition => value_1 + value_2,
        Calculation::Subtraction => value_1 - value_2,
        Calculation::Multiplication => value_1 * value_2,
        Calculation::Division => value_1 / value_2,
    }
}
