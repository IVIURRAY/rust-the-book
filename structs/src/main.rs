struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("user123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!(
        "Username {} Email {} Active {} Counter {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    let user2 = build_user(String::from("a@b.com"), String::from("abc"));

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: user2.email,
        sign_in_count: user2.sign_in_count,
    };

    let _user4 = User {
        active: false,
        ..user3
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    calculate_area_program();
}

fn build_user(email: String, username: String) -> User {
    // Shorthand we can omit the variables.
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        (self.width > rect.width) & (self.height > rect.height)
    }

    // Associated Function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn calculate_area_program() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect1)
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
