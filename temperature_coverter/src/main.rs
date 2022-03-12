use std::io;

fn main() {
    println!("Enter a temperature in Fahrenheit to convert to Celcius:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let input: f64 = temp
        .trim()
        .parse()
        .expect("Input must be a number");

    let conversion = round_two_decimals(fahrenheit_to_celcius(input));

    println!("{}°F is {}°C", input, conversion);
}

fn fahrenheit_to_celcius(num: f64) -> f64 {
    (num - 32.0) * (5.0 / 9.0)
}

fn round_two_decimals(num: f64) -> f64 {
    f64::trunc(num * 100.0) / 100.0
}
