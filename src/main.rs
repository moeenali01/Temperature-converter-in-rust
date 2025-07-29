use std::io;

fn main(){
    println!("Temperature Converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("Choose an option (1 or 2):"); 

    let mut choise= String::new();
    io::stdin().read_line(&mut choise).expect("Failed to read line");
    let choise: u32 = match choise.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter 1 or 2.");
            return;
        }
    };

    if choise==1{
        celcius_to_fahrenheit();
    } else if choise==2{
        fahrenheit_to_celcius();
    } else{
        println!("Invalid input, please enter 1 or 2.");
    }
}

fn celcius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read line");
    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return;
        }
    };
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{} Celsius is {} Fahrenheit", celsius, fahrenheit);
}

fn fahrenheit_to_celcius() {
    println!("Enter temperature in Fahrenheit:");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return;
        }
    };
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);
}