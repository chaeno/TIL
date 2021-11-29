fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("주목해 주세요! {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T:Display
{
    println!("주목하세요: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 모든 문자열 리터럴은 'static 수명이다.
    // 전체 프로그램에 적용되고 프로그램의 바이너리에 직접 저장된다.
    let s: &'static str = "문자열은 정적 수명이다.";

    let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계에....");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("문장에서 마침표'.'를 찾을 수 없습니다.");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    let r;
    {
        let x = 5;
        r = &x;
    }

    // x는 범위를 벗어나서 유효하지 않다.
    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {}", result);

    let string1 = String::from("아주 아주 긴 문자열");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("더 긴 문자열: {}", result);
    }

    let string1 = String::from("아주 아주 긴 문자열");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // string2가 유효하지 않을 수 있으므로 컴파일이 허용되지 않는다.
    // 위 케이스의 경우 string1이 리턴될 것이므로 string2는 상관 없지 않냐고 생각할 수 있지만,
    // 컴파일러는 런타임에 결정되는 사항을 알 수 없다.
    // println!("더 긴 문자열: {}", result);
}
