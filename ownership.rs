/*
 Ownership rules in Rust

 1. Each value in Rust has an owner.
 2. There can only be one owner at a time.
 3. When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world!"); // error as s1 is out of scope, ownership transferred.

    // Reference example
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // Mutable reference example
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Reference rules in Rust
/*
 1. At any given time, you can have either one mutable reference or any number of immutable references.
 2. References must always be valid.
*/

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but s no longer has any content that we
               // could meaningfully use with the value 5, so word is now totally invalid!
}

// String Slices
// A string slice is a reference to a contiguous sequence of the elements of a String , and it looks like this:
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11]; // With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods. [..5].

