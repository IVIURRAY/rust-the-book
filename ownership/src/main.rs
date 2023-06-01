// fn main() {
//     let s = String::from("hello"); // s comes into scope
//
//     takes_ownership(s); // s's value moves into the function...
//                         // ... and so is no longer valid here
//
//     let x = 5; // x comes into scope
//
//     makes_copy(x); // x would move into the function,
//                    // but i32 is Copy, so it's okay to still
//                    // use x afterward
//
//     // println!("{s}"); // Unable to print s here as it has been `drop` by takes_ownership.
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.
//
// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.
//
// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// ================================================

// Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, len);
// }
//
// fn calculate_length(s: &String) -> usize {
//     // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, it is not dropped.

// ================================================

// In the event that you want to change a reference to a variable you need to tell Rust that this reference can be mutated.
// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code will fail.
// let mut s = String::from("hello");
//
// let r1 = &mut s;
// let r2 = &mut s;
//
// println!("{}, {}", r1, r2);

fn main() {
    let mut s = String::from("hello");
    println!("S is: {s}");

    change(&mut s);

    println!("S is now: {s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
