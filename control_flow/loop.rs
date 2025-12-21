// A continuous loop
fn main() {
    loop {
        println!("again!"); // infinite loop
    }
}

// Returning a value from a loop
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
        break counter * 2; // this line stops the loop and returns the value after the break keyword
        }
    };
    println!("The result is {result}");
}

// Loop in loop
fn main() {
    let mut count = 0;
    // labeled loop syntax
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

// while loop
fn main() {
    let mut number = 4;

    while number > 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Done");
}

// for loop
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}
