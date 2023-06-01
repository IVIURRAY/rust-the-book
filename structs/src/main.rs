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
