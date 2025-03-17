fn sample() {
    // こんにちは！
    let s = Some(String::from("Hello!"));

    if let Some(ref _s) = s {
        // 文字列が見つかりました
        println!("found a string");
    }

    println!("{:?}", s);
}

fn modify() {
    // こんにちは！
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        // 文字列が見つかりました
        println!("found a string");
    }

    println!("{:?}", s);
}

fn main() {
    sample();
    modify();
}
