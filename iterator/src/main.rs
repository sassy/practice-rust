fn map_test() {
    let v = vec![1, 2, 3, 4];
    let v2 = v.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("{:?}", v2);
}

fn filter_test() {
    let v = vec![1, 2, 3, 4];
    let v2 = v.iter().filter(|&x| x % 2 == 0).collect::<Vec<&i32>>();
    println!("{:?}", v2);
}

fn filter_map_test() {
    let v = vec![1, 2, 3, 4];
    let v2 = v
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect::<Vec<i32>>();
    println!("{:?}", v2);
}

fn collect_test() {
    let v = vec!["A", "B", "C"];
    let v2 = v.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    println!("{:?}", v2);
}

fn sum_test() {
    let v = vec![1, 2, 3, 4];
    let s: i32 = v.iter().sum();
    println!("{}", s);
}

fn product_test() {
    let v = vec![1, 2, 3, 4];
    let p: i32 = v.iter().product();
    println!("{}", p);
}

fn count_test() {
    let v = vec![1, 2, 3, 4];
    let c = v.iter().filter(|&x| x % 2 == 0).count();
    println!("{}", c);
}

fn any_test(v: Vec<i32>) -> bool {
    v.iter().any(|&x| x % 2 == 0)
}

fn main() {
    println!("Hello, world!");
    map_test();
    filter_test();
    filter_map_test();
    collect_test();
    sum_test();
    product_test();
    count_test();
    println!("{}", any_test(vec![1, 2, 3, 4]));
    println!("{}", any_test(vec![1, 3, 5, 7]));
}
