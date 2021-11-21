fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); (error: value borrowed here after move)
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); (error: value borrowed here after move)
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s1);
    // println!("{}", s2); (error: value borrowed here after move)
    println!("{}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("'{}'의 길이는 {}입니다.", s2, len);

    let s1 = String::from("hello");
    let len = calculate_length_reference(&s1);
    println!("'{}'의 길이는 {}입니다.", s1, len);

    let s = String::from("hello");
    change(&s);

    let mut s = String::from("hello");
    change_mutable(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // error: second mutable borrow occurs here
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1)
    }
    let r2 = &mut s;
    println!("{}", r2);

    let mut s = String::from("hello");
    let r1 = &s; // immutable borrow occurs here
    let r2 = &s; // immutable borrow occurs here
    // error: mutable borrow occurs here
    // let r3 = &mut s;
    // println!("{}, {}. and {}", r1, r2, r3)

    // let reference_to_nothing = dangle();
    println!("{}", no_dangle())
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let lengh = s.len();
    (s, lengh)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn change(_some_string: &String) {
    // if this is intentional, prefix it with an underscore: `_some_string`
    // some_string.push_str(", world")
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // error: expected named lifetime parameter
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}