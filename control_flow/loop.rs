// A continuous loop

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

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