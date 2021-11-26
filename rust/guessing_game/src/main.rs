use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "유추한 값은 반드시 1에서 100 사이의 값이어야 합니다. 입력한 값:{}",
                value
            );
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("숫자를 맞혀봅시다.");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("사용자가 맞혀야 할 숫자: {}", secret_number);

    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");

        println!("입력한 값: {}", guess);

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}
