use std::io;

fn main() {
    println!("Wellcome to Celcius Fahrenheit Converter App!");
    start();
}

fn start() {
    println!("Please select one of the following options (F/C):");
    println!("1. Celcius to Fahrenheit (F)");
    println!("2. Fahrenheit to Celcius (C)");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    if option.trim() == "F" {
        println!("Please enter the temperature in Celcius:");

        let mut celcius = String::new();

        io::stdin()
            .read_line(&mut celcius)
            .expect("Failed to read line");

        let celcius: f64 = celcius.trim().parse().unwrap_or(0.0);

        let fahrenheit = convert_to_fahrenheit(celcius);
        println!("The temperature in Fahrenheit is: {}", fahrenheit);

        start();

    } else if option.trim() == "C" {
        println!("Please enter the temperature in Fahrenheit:");

        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: f64 = fahrenheit.trim().parse().unwrap_or(0.0);

        let celcius = convert_to_celsius(fahrenheit);
        println!("The temperature in Celcius is: {}", celcius);

        start();
        
    } else {
        println!("Invalid option");
        start();
    }
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
