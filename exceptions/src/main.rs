use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // calling_panic();

    // File open returns a Result. If the file is not there we will get Err otherwise we get Ok
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => panic!("Problem opening file {:?}", other_error),
        },
    };

    let greeting_file = greeting_file.unwrap();
}

fn calling_panic() {
    // panic!("I throw a panic, crash and burn program!");

    let v = vec![1, 2, 3];

    v[99]; // This code will panic!
}

use std::io::{self, Read};
// These two function are the same and are propagating the errors up.
fn read_username_from_file_1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// The ? char will return early the error if we find one.
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
