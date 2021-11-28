use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new (x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("가장 큰 멤버는 x: {}", self.x);
        } else {
            println!("가장 큰 멤버는 y: {}", self.y);
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("{}님의 기사 더 읽기", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    // 구현을 비워두면 기본 함수가 수행된다.
    // fn summarize(&self) -> String {
    //     format!("{}, by {}, ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("속보! {}", item.summarize());
}

// trait 경계
pub fn notify_full<T: Summary>(item: &T) {
    println!("속보! {}", item.summarize());
}

// impl trait
// pub fn notify(item1: impl Summary, item2: impl Summary) {}

// item1과 item2가 반드시 동일한 타입으로 강제하기 위해서는 trait 경계 사용
// pub fn notify<T: Summary>(item1: T, item2: T) {}

// + 문법은 하나 이상의 trait 구현을 요구할 때 사용
// pub fn notify(item: impl Summary + Display) {}
// pub fn notify<T: Summary + Display>(item: T) {}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

// 위 함수를 where 절을 이용하여 아래와 같이 정의 가능
// fn soe_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug,
// {}

// 주의! 하나의 Summary 타입만 리턴해야한다.
// Summary trait를 구현한 A 또는 B 타입을 리턴하는 코드는 일반적으로 에러가 발생한다.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("러스트 언어 공부를 시작했습니다."),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("러스트 언어 공부를 시작했습니다."),
        reply: false,
        retweet: false,
    };
    println!("새 트윗 1개: {}", tweet.summarize());
    notify(&tweet);
    notify_full(&tweet);

    let article = NewArticle {
        headline: String::from("대한민국, 러시아 월드컵 예선에서 독일을 이겼다."),
        location: String::from("카잔 아레나, 러시아"),
        author: String::from("위키백과"),
        content: String::from("2018년 ..."),
    };
    println!("새로운 기사: {}", article.summarize());
    notify(&article);
    notify_full(&article);
}
