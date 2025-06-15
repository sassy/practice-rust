fn numbers_3times() {
    let numbers = vec![1, 4, 7];
    let results = numbers.iter().map(|x| x * 3).collect::<Vec<_>>();
    println!("{:?}", results);
}

fn word_lengths() {
    let words = vec!["cat", "elephant", "dog"];
    let lengths = words.iter().map(|x| x.len()).collect::<Vec<_>>();
    println!("{:?}", lengths);
}

fn numbers_even_2times() {
    let numbers = vec![10, 15, 20, 25, 30];
    let evens = numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 2)
        .collect::<Vec<_>>();
    println!("{:?}", evens);
}

fn main() {
    numbers_3times();
    word_lengths();
    numbers_even_2times();
}
