use std::io;

fn convert_temp(temp_in_celsius: i32) -> i32 {
    let temp_in_fahrenheit = (temp_in_celsius * 9 / 5) + 32;
    return temp_in_fahrenheit;
}

fn main() {
    println!("Convert Celsius into Fahrenheit!!");
    println!("Enter temperature in celsius");
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read lines");

        match input.trim().parse::<i32>() {
            Ok(n) => {
                let temp_in_celsius: i32 = n;

                let temp_in_fahrenheit = convert_temp(temp_in_celsius);

                println!("Temperature in fahrenheit: {temp_in_fahrenheit}");

                break;
            }
            Err(e) => {
                println!("Invalid input: {}. Please enter a valid integer.", e);
            }
        }
    }
}
