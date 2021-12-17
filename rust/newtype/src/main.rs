type Kilometers = i32;
type Trunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("안녕하세요"));
    let f: Trunk = Box::new(|| println!("안녕하세요"));
}

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {

}

fn takes_trunk_type(f: Trunk) {

}

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // 
// }

// fn returns_trunk_type() -> Trunk {
    // 
// }