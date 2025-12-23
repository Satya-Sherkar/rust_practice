use std::io;

fn convert_temp(temp_in_celcius: i32) -> i32 {
    let temp_in_fahrenheit = (temp_in_celcius * 9 / 5) + 32;
    return temp_in_fahrenheit;
}

fn main() {
    println!("Convert Celcius into Fahrenheit!!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read lines");

    let temp_in_celcius: i32 = input.trim().parse().expect("Failed to parse");

    let temp_in_fahrenheit = convert_temp(temp_in_celcius);

    println!("Temperature in fahrenheit: {temp_in_fahrenheit}");
}
