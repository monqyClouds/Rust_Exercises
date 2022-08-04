use std::io;

fn main() {
    println!("Are you converting from 'c' or 'f'?");
    
    let mut base = String::new();
    let mut deg = String::new();

    io::stdin()
        .read_line(&mut base)
        .expect("Error reading line");

    let base = base.trim();

    println!("What numerical value of the base are you converting");

    io::stdin()
        .read_line(&mut deg)
        .expect("Error reading line");

    let deg: f64 = deg
        .trim()
        .parse()
        .expect("Invalid number");

    if base == "c" {
        let result = celcius_to_fahrenheit(deg);
        println!("{} degree celcius is equivalent to {} degree farenheit", deg, result);
    } else {
        let result = fahrenheit_to_celcius(deg);
        println!("{} degree farenheit is equivalent to {} degree celcius", deg, result);
    }
}

fn celcius_to_fahrenheit(deg: f64) -> f64 {
    (deg * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celcius(deg: f64) -> f64 {
    (deg - 32.0) * 5.0 / 9.0
}
