use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("안녕하세요 기장입니다.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("날아라!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*양팔을 펄떡거린다*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("점박이")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("멍멍이")
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("안녕하세요"),
                    String::from("러스트입니다.")]);
    println!("w = {}", w);

    let point = Point{x:1, y:3};
    point.outline_print();

    println!("새끼 강아지 이름은 {}", Dog::baby_name());
    println!("새끼 강아지 이름은 {}", <Dog as Animal>::baby_name());

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    assert_eq!(Point {x:1, y:0} + Point {x:2, y:3}, Point {x:3, y:3});
}
