fn create_vector() -> Vec<i32> {
    let mut values = Vec::new();
    for n in 1..10 {
        values.push(n)
    }
    values
}

fn print_vector(nums: Vec<i32>) {
    for n in nums {
        println!("{}", n)
    }
}

fn main() {
    let ret = create_vector();
    print_vector(ret);
}
