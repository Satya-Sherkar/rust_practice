use std::io;

fn get_fibonacci(num: usize) -> usize {
    if num == 0 || num == 1 {
        return num;
    } else {
        //TODO: Implement logic here
    }
}

fn main() {
    println!("Get the nth fibonacci sequence number...");
    println!("Please enter the nth number..");

    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read lines");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                let nth_term = get_fibonacci(num);
                println!("The {}th fibonacci number is:{}", input, nth_term);
                break;
            }

            Err(err) => {
                println!("Please enter a valid number. {err}");
            }
        }
    }
}
