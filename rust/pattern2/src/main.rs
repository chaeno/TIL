fn foo(_: i32, y: i32) {
    println!("이 함수는 y 매개변수만 사용한다.: {}", y);
}

fn main() {
    foo(3, 4);

    // let mut setting_value = None;
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("이미 설정된 값을 덮어쓸 수 없습니다.");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("{:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("일치하는 숫자: {}, {}, {}", first, third, fifth);
        }
    }

    let _x = 5;
    let y = 10;

    let s = Some(String::from("안녕하세요"));
    // if let Some(_s) = s { 로 사용 시 소유권이 _s로 바인딩하므로 s를 다시 사용할 수 없다.
    if let Some(_) = s {
        println!("문자열을 찾았습니다.");
    }
    println!("{:?}", s);

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x = {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("first = {}, last = {}", first, last);
        }
        // .. 의 사용이 모호하면 에러 발생
        // (.., second, ..) => {
            // println!("second = {}", second);
        // }
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("5보다 작은 값: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("일치, n = {:?}", n),
        _ => println!("일치하지 않음, x = {:?}", x),
    }
    println!("결과: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("예"),
        _ => println!("아니요"),
    }

    enum Message {
        Hello {id: i32},
    }
    let msg = Message::Hello{ id: 5};
    match msg {
        Message::Hello { id: id_variable @ 3..=7} => {
            println!("id를 범위에서 찾았습니다: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("id를 다른 범위에서 찾았습니다.")
        }
        Message::Hello { id } => {
            println!("다른 id {}를 찾았습니다.", id)
        }
    }

}
