fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    // s 문자열은 ""로 비워지고 index 값만 남아 앞으로 에러의 소지가 있으나 컴파일러가 찾아내지 못함
    println!("{}", word);

    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..s.len()];
    let hello_world = &s[..];
    println!("{}", hello);
    println!("{}", world);
    println!("{}", hello_world);

    let mut s = String::from("hello world");
    let word = first_word_new(&s);
    // s.clear();
    // word가 s에 대한 immutable 참조를 가지므로 s.clear()에서 mutable 참조를 할 수 없다.
    println!("{}", word);

    println!();
    let my_string = String::from("hello world");
    let word = first_word_slice(&my_string[..]);
    println!("{}", word);
    let my_string_literal = "hello world";
    let word = first_word_slice(&my_string_literal[..]);
    println!("{}", word);
    let word = first_word_slice(my_string_literal);
    println!("{}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_new(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}