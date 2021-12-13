fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("선호하는 {}색을 배경으로 사용합니다.", color);
    } else if is_tuesday {
        println!("화요일엔 녹색이죠!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("보라색을 배경으로 사용합니다.");
        } else {
            println!("오랜지색을 배경으로 사용합니다.");
        }
    } else {
        println!("파란색을 배경으로 사용합니다.");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("인덱스 {}의 값: {}", index, value);
    }

    let (x, y, _) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    let some_option_value = Some(5);
    // let은 부인할 수 없는 값을 받아야하므로 Option<i32>에 해당하는 값을
    // Some(x)로 받을 경우 None을 받아 낼 수 없으므로 에러 처리된다.
    // let Some(x) = some_option_value;
    // if let Some(x)은 None일 경우 중괄호 안의 코드를 실행하지 않는 것으로 모든 값을 커버한다.
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // warning 발생: 항상 match되므로 if let은 useless하다.
    // let만 쓰면 된다.
    if let x = 5 {
        println!("{}", x)
    }

    let x = 7;
    match x {
        1 => println!("하나"),
        2 => println!("둘"),
        3 => println!("셋"),
        _ => println!("나머지"),
    }

    let x = None;
    let y = 10;
    match x {
        Some(50) => println!("50"),
        Some(y) => println!("일치, y = {:?}", y),
        _ => println!("일치하지 않음, x = {:?}", x),
    }
    println!("결과: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("1 또는 2"),
        3 => println!("3"),
        _ => println!("그 외 나머지 값"),
    }

    let x = 5;
    match x {
        // 책에서 나오는 1 ... 5는 deprecated 됨
        1..=5 => println!("1에서 5 중 하나"),
        _ => println!("그 외 나머지 값"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("ASCII 문자의 전반부"),
        'k'..='z' => println!("ASCII 문자의 후반부"),
        _ => println!("그 외 나머지 값"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // 구조체 해체 시 동일 이름의 변수로 더 간단히 사용할 수 있다.
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("x 축 {}에 위치하는 점", x),
        Point { x: 0, y } => println!("y 축 {}에 위치하는 점", y),
        Point { x, y } => println!("좌표 ({}, {})에 위치하는 점", x, y),
    }

    struct Color(i32, i32, i32);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("Quit: 해체할 값이 없습니다.");
        }
        Message::Move { x, y } => {
            println!("Move: x = {}, y = {}", x, y);
        }
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(Color(r, g, b)) => {
            println!("ChangeColor: R = {}, G = {}, B = {}", r, g, b);
        }
    }

    enum Color2 {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32)
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color2),
    }

    let msg = Message2::ChangeColor(Color2::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color2::Rgb(r, g, b,)) => {
            println!("ChangeColor: R = {}, G = {}, B = {}", r, g, b);
        },
        Message2::ChangeColor(Color2::Hsv(h, s, v)) => {
            println!("ChangeColor: H = {}, S = {}, V = {}", h, s, v);
        },
        _ => {}
    }

    let ((feet, inches), Point {x, y}) = ((3, 10), Point {x: 3, y: -10});
    println!("{}, {}, {}, {}", feet, inches, x, y);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("현재 위치: ({}, {})", x, y);
}
