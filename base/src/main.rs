
fn kurikaeshi(v: &Vec<i32>) {
    for i in v {
        print!("{} ", i);
    }
    println!("");
}

fn kurikaeshi_iter(v: &Vec<i32>) {
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");
}

fn kurikaeshi_iter_with_idx(v: &Vec<i32>) {
    for (i, x) in v.iter().enumerate() {
        print!("{}:{} ", i, x);
    }
    println!("");
}

fn test(x: i32, y: i32) -> bool {
    x > y
}

// if文の例
fn condition(a :i32, b: i32) {
    if a == b {
        println!("a == b is ok");
    } else if test(a, b) {
        println!("a < b is ok");
    } else {
        println!("a < b is ng");
    }
}

fn main() {
    condition(10, 20);
    condition(20, 10);
    condition(10, 10);
    let v = vec![10,20, 30];
    kurikaeshi(&v);
    kurikaeshi_iter(&v);
    kurikaeshi_iter_with_idx(&v);
}
