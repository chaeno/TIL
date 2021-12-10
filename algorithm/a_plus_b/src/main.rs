use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut sum = 0;
    for element in buffer.split_whitespace().map(|s| s.trim().parse::<i32>().unwrap()) {
        sum += element;
    }

    println!("{}", sum);
}
