use std::io;

fn main() {
    let mut choice = String::new();

    println!("Temperature conversion for Fahrenheit and Celsius");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    println!("Choose your operation, 1 or 2");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    println!("Temperature ?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temp: f32 = match input.trim().parse() {
        Ok(num) => num,
        _ => 0,
    } as f32;

    if choice == "1\n" {
        let celsius = f_to_c(temp);
        println!("Celsius temperature is {celsius}");

    } else {
        let farhenheit = c_to_f(temp);
        println!("Farhenheit temperature is {farhenheit}");
    }
    println!();
}

fn f_to_c(value: f32) -> f32 {
    (5.0/9.0) * (value - 32.0)
}

fn c_to_f(value: f32) -> f32 {
    (9.0/5.0) * value + 32.0
}