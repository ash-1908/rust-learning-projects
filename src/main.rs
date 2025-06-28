use std::io;

fn main() {
    println!("Celsius to Farenheit Converter");

    loop {
        println!("1. Celsius To Farenheit");
        println!("2. Farenheit to Celsius");
        println!("3. Exit");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Error occured while input.");
        
        let user_input : u8 = match user_input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("Invalid input. Please try again.");
                continue;
            }
        };

        if user_input == 3 {
            break;
        } else if user_input > 3 || user_input < 1{
            println!("Invalid option selected.");
            continue;
        }

        println!("Enter temperature: ");
        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature).expect("Error occured while input.");
        let temperature : f32 = match temperature.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("Invalid temperature.");
                break;
            }
        };

        if user_input == 1 {
            println!("Temperature in Farenheit is: {}", celsius_to_farenheit(temperature));
        } else if user_input == 2 {
            println!("Temperature in Celsius is: {}", farenheit_to_celsius(temperature));
        }
    }
}

fn celsius_to_farenheit(temp_celsius : f32) -> f32 {
    (1.8 * temp_celsius) + 32.0
}

fn farenheit_to_celsius(temp_farenheit : f32) -> f32 {
    (temp_farenheit - 32.0) * (5.0/9.0)
}