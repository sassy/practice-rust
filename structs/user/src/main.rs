struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //shorthand
        email,  //shothand
        sign_in_count: 1
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someoneuser"),
      active: true,
      sign_in_count: 1
    };

    let mut user2 = build_user(String::from("anyone@example.com"), String::from("testuser"));

    user2.email = String::from("anotheremail@example.com");

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    println!("{} {}", user1.username, user1.email);
    println!("{} {}", user3.username, user3.email);

    let mut a = Point {x: 1, y: 2};
    a.x += 1;
    let b = Point {y: 1, ..a};
    a.x += 1;
    println!("{}", a.x);
    println!("{}", b.x);
}
