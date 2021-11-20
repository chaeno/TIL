fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINT의 값: {}", MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x의 값: {}", x);

    let space = "       ";
    let space = space.len();
    println!("space의 값: {}", space);

    let guess: usize = "42".parse().expect("숫자가 아닙니다!");
    println!("guess의 값: {}", guess);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("x의 값, y의 값: {}, {}", x, y);

    let t = true;
    let f: bool = false;
    println!("t의 값, f의 값: {}, {}", t, f);

    let c = 'z';
    println!("c의 값: {}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup (0, 1, 2)의 값: ({}, {}, {})", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("tup (x, y, z)의 값: ({}, {}, {})", x, y, z);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let error_index = 10; // runtime panic (index out of bounds)
    let index = 4;
    println!(
        "a의 값: [{}, {}, {}, {}, {}]",
        a[0], a[1], a[2], a[3], a[index]
    );

    another_function();
    another_function_with_parameter(5, 2);

    let x = 5;
    let y = {
        let x = 3; // statement (semicolon)
        x + 1 // expression (not semicolon)
    };
    println!("x, y의 값: {}, {}", x, y);

    let x = five();
    println!("x의 값: {}", x);
    println!("plus_one의 값: {}", plus_one(x))
}

fn another_function() {
    println!("또 다른 함수");
}

fn another_function_with_parameter(x: i32, y: i16) {
    println!("argument x, y의 값: {}, {}", x, y)
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
