use std::io;

pub fn run() {
    println!("Choose conversion direction:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    // Get user input for the conversion choice
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    match choice {
        1 => {
            // Celsius to Fahrenheit conversion
            println!("Enter temperature in Celsius:");
            let mut temp_celsius = String::new();
            io::stdin()
                .read_line(&mut temp_celsius)
                .expect("Failed to read line");

            let temp_celsius: f64 = temp_celsius
                .trim()
                .parse()
                .expect("Please enter a valid number");

            let temp_fahrenheit = (temp_celsius * 9.0 / 5.0) + 32.0;
            println!("{}°C is equal to {}°F", temp_celsius, temp_fahrenheit);
        }
        2 => {
            // Fahrenheit to Celsius conversion
            println!("Enter temperature in Fahrenheit:");
            let mut temp_fahrenheit = String::new();
            io::stdin()
                .read_line(&mut temp_fahrenheit)
                .expect("Failed to read line");

            let temp_fahrenheit: f64 = temp_fahrenheit
                .trim()
                .parse()
                .expect("Please enter a valid number");

            let temp_celsius = (temp_fahrenheit - 32.0) * 5.0 / 9.0;
            println!("{}°F is equal to {}°C", temp_fahrenheit, temp_celsius);
        }
        _ => {
            println!("Invalid choice! Please select 1 or 2.");
        }
    }
}
