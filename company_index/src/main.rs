// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
// department or all people in the company by department, sorted alphabetically.

use std::io;

fn main() {
    println!("Welcome to the Globex Corporation employee index");
    println!("================================================\n");

    println!("Please input a command:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let words = input
        .trim()
        .split(' ')
        .collect::<Vec<_>>();

    println!("{:?}", words);

    for word in words {
        println!("{}", word);
    }

}
