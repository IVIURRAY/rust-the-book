use measurements::Mass;

fn main() {
    println!("Hello, world!");

    let mass = Mass::from_kilograms(75.5);

    println!("I weight {mass} or {}", mass.as_stones());
}
