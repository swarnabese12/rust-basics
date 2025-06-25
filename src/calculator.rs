use std::io;

pub fn run() {
    println!("🧮 Calculator Module");

    loop {
        println!("Enter first number:");
        let num1 = read_number();

        println!("Enter operator (+, -, *, /):");
        let op = read_operator();

        println!("Enter second number:");
        let num2 = read_number();

        let result = match op {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => {
                if num2 == 0.0 {
                    println!("❌ Error: Division by zero");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("❌ Invalid operator");
                continue;
            }
        };

        println!("Result: {} {} {} = {}", num1, op, num2, result);

        println!("Do you want to calculate again? (yes/no):");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if choice.trim().to_lowercase() != "yes" {
            println!("👋 Exiting calculator.");
            break;
        }
    }
}

fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => println!("❗ Please enter a valid number:"),
        }
    }
}

fn read_operator() -> char {
    loop {
        let mut op = String::new();
        io::stdin().read_line(&mut op).unwrap();
        let trimmed = op.trim();
        if trimmed.len() == 1 {
            return trimmed.chars().next().unwrap();
        }
        println!("❗ Enter a valid operator (+, -, *, /):");
    }
}
