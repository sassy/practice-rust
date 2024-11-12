fn main() {
    let some_value = Some(String::from("Hello"));
    let none_value: Option<String> = None;

    println!("{}", response(some_value));
    println!("{}", response(none_value));
}

fn response(value: Option<String>) -> String {
    match value {
        Some(value) => {
            value + "World"
        }
        None => String::from("No value")
    }
}
