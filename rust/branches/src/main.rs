fn main() {
    let number = 7;

    if number < 5 {
        println!("조건이 일치합니다.");
    } else {
        println!("조건이 일치하지 않습니다.");
    }

    if number != 0 {
        println!("변수에 저장된 값이 0이 아닙니다.")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // error: if and else have incompatibe types
    // let number = if condition { 5 } else { "six" };
    println!("number의 값: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result의 값: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("발사!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("요소의 값 {}", a[index]);
        index += 1;
    }
    for (i, element) in a.iter().enumerate() {
        println!("요소의 인덱스, 값: {}, {}", i, element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("발사!");
}
