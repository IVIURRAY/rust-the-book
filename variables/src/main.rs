fn variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // ints
    let y: u8 = 5;

    // floats
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // booleans
    let f: bool = false; // with explicit type annotation

    // chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tup is: {x} {y} {z}");
    let a = tup.1;
    println!("The value of tup is: {a}");

    // array
    let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    // let a = [3; 5]; == let a = [3, 3, 3, 3, 3];
    let b = a[1];
    println!("Accessing an array element {b}")
}

fn functions() {
    another_function(12);
    expression();
    let mut x = return_with_statement();
    println!("x is {x}");
    x = return_with_expression();
    println!("x is {x}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 10 // note: no semi colon so this is an expression and will return
    };

    println!("The value of y is {y}");
}

fn return_with_statement() -> i32 {
    let x = 5;
    return x;
}

fn return_with_expression() -> i32 {
    5 // without a semicolon the function will return this value
      // if you put a semicolon here the code will error because the fn will not be returning a value.
}

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // if number {
    // This will fail as it expects a bool, not a integer.
    //     println!("number was something other than zero");
    // }
    if number != 0 {
        println!("number was something other than zero");
    }

    let condition = true;
    let favourite_number = if condition { 5 } else { 6 };
    println!("My favourite number is {favourite_number}");
}

fn loops() {
    // infinite loop
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    let mut count = 0;
    // this is a loop label, so you can break out of a specific loop within an inner loop.
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

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn main() {
    println!("===== VARIABLES =====");
    variables();
    println!("===== FUNCTIONS =====");
    functions();
    println!("===== CONTROL FLOW =====");
    control_flow();
    println!("===== LOOPS =====");
    loops();
}
