use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    println!("{}", iter.next().unwrap() as f64 / iter.next().unwrap() as f64);
}
