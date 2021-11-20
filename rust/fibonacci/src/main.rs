fn main() {
    for i in 0..6 {
        println!("f({}): {}", i, fibo(i))
    }
}

fn fibo(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibo(n - 1) + fibo(n - 2)
}
