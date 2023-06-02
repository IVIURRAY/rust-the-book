#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // Quarts have unique designs to their US state.
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter for {:?}", state);
            25
        }
    }
}

fn main() {
    let c = Coin::Penny;
    let value = value_in_cents(c);
    println!("Coin value in cents: {value}");

    let c = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(c);
    println!("Coin value in cents: {value}");

    add_one();
    dice_roll();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_one() {
    let five = Some(5);
    let six = plus_one(five);
    println!("Value of {:?} plus 1 = {:?}", five, six);
    let none = plus_one(None);
    println!("Value of None plus 1 = {:?}", none);
}

fn dice_roll() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The last arm in a match statement matches all and will be called.
        other => move_player(other),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The _ arm is used when you don't want to use the user input value.
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // or even just do nothing
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
}
