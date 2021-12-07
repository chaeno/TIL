use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("안녕하세요 {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // 역참조 강제가 없다면 아래와 같이 호출해야함
    hello(&(*m)[..]);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = &x;

    println!("{}", x);
    // stack 주소
    println!("{:p}", y);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    println!("{}", x);
    // heap 주소
    println!("{:p}", y);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
