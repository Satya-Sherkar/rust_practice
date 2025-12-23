use std::io;

fn get_fibonacci(num: u64) -> u64 { 
    if num == 0 || num == 1 {
        return num;
    } else {
        let mut a: u64 = 0;
        let mut b: u64 = 1;

        for _ in 2..=num {
            let next = a + b;
            a = b;
            b = next;
        }
        b // The last expression in a function without a trailing semicolon is the return value.
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

        match input.trim().parse::<u64>() {
            Ok(num) => {
                let nth_term = get_fibonacci(num);
                println!("The fibonacci number at position {input} is: {nth_term}");
                break;
            }

            Err(err) => {
                println!("Please enter a valid number. {err}");
            }
        }
    }
}
