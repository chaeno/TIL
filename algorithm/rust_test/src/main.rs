use std::io;

struct Inputs {
    first: String,
    second: String,
}

impl Inputs {
    fn new() -> Inputs {
        let mut first = String::new();
        let mut second = String::new();

        io::stdin().read_line(&mut first).unwrap();
        io::stdin().read_line(&mut second).unwrap();        

        Inputs {
            first: String::from(first.trim()),
            second: String::from(second.trim()),
        }
    }

    fn print_result(self) {
        let first_u32 = self.first.parse::<u32>().unwrap();
        let second_u32 = self.second.parse::<u32>().unwrap();

        let second_chars_iter = self.second.chars().map(|c| {c.to_digit(10).unwrap()});
        for c in second_chars_iter.rev() {
            println!("{}", first_u32 * c);
        }
        println!("{}", first_u32 * second_u32);
    }
}

fn main() {
    let input = Inputs::new();
    input.print_result();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_is_test() {
        let input  = Inputs {
            first: String::from("472"),
            second: String::from("385"),
        };

        input.print_result();
        assert_eq!(1, 1);
    }
}

