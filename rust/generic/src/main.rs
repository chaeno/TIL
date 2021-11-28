fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// binary operation `>` cannot be applied to type `T`
// std::cmp::PartialOrd 트레이트 구현 요구됨
fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_generic_without_copy_trait<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_index = 0;
    for i in 0..list.len() {
        if list[i] > list[largest_index] {
            largest_index = i;
        }
    }
    &list[largest_index]
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Point { x: 5, y: 10 };
    println!("integer.p - {}", integer.x());
    let float = Point { x: 1.0, y: 4.0 };
    // expected integer, found floating-point number
    // x, y는 모두 같은 타입이여야 한다.
    // let wont_work = Point { x: 5, y: 4.0 };
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("가장 큰 숫자: {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("가장 큰 숫자: {}", largest);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("가장 큰 숫자: {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("가장 큰 숫자: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("가장 큰 문자: {}", result);

    let result = largest_generic(&number_list);
    println!("*가장 큰 숫자: {}", result);
    let result = largest_generic(&char_list);
    println!("*가장 큰 문자: {}", result);

    let result = largest_generic_without_copy_trait(&number_list);
    println!("**가장 큰 숫자: {}", result);
    let result = largest_generic_without_copy_trait(&char_list);
    println!("**가장 큰 문자: {}", result);
}
