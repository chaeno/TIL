use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("안녕하세요 매크로! 내 이름은 Pancakes입니다!");
    }
}

fn main() {
    Pancakes::hello_macro();
}