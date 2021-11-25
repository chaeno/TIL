fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];
    } // 변수 v가 범위를 벗어나면 drop 메서드가 호출되어 메모리가 해제된다.

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("세 번째 원소: {}", third);
    match v.get(2) {
        Some(third) => println!("세 번째 원소: {}", third),
        None => println!("세 번째 원소가 없습니다."),
    }

    // panic
    // let does_not_exist = &v[100];

    // return None
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // first를 사용하려하면 에러 발생
    // 벡터 크기가 충분하지 않아 새로운 메모리 할당이 필요한데,
    // 변수가 이전 메모리의 값을 참조하고 있으면 해당 값을 확실히 보장할 수 없다.
    // println!("{}", first)

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("블루")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Float(10.15),
    ];

    let mut s = String::new();

    let data = "문자열 초깃값";
    let s = data.to_string();
    // 문자열 리터럴의 to_string() 메서드를 직접호출 할 수 있다.
    let s = "문자열 초깃값".to_string();
    let s = String::from("문자열 초깃값");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // s1은 소유권이 해제되어 사용할 수 없다.
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s1 = String::from("hello");
    // String에 대한 index 접근은 허용하지 않음
    // UTF-8 인코딩 (1~3byte)되어 있으므로 정확한 정보를 전달 할 수 없기 때문
    // let h = &s1[0];

    let hello = "안녕하세요";
    let s = &hello[0..3];
    // panic
    // let s1 = &hello[0..2];
    println!("{}", s);

    for c in "안녕하세요".chars() {
        println!("{}", c);
    }
    for b in "안녕하세요".bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("블루"), 10);
    // scores.insert(String::from("블루"), 15);
    // scores.insert(String::from("옐로"), 50);
    scores.entry(String::from("옐로")).or_insert(50);
    scores.entry(String::from("블루")).or_insert(50);

    let team_name = String::from("블루");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("{:?}", scores);

    let teams = vec![String::from("블루"), String::from("옐로")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("블루");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 소유권이 이미 map으로 넘어 갔으므로 field_name, field_value는 사용할 수 없다.
    // i32와 같이 Copy trait가 구현된 값들은 해당하지 않는다.
    // println!("{}, {}", field_name, field_value);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:?}", map)
}
