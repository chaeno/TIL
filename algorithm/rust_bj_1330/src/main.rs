use std::io;

struct Inputs {
    a: i32,
    b: i32,
}

impl Inputs {
    fn new() -> Inputs {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        let mut inputs_iter = inputs.split_whitespace().map(|x| x.parse::<i32>().unwrap());

        Inputs {
            a: inputs_iter.next().unwrap(),
            b: inputs_iter.next().unwrap(),
        }
    }

    fn print_result(&self) {
        if self.a > self.b {
            println!(">");
        } else if self.a < self.b {
            println!("<");
        } else {
            println!("==");
        }
    }
}

fn main() {
    let inputs = Inputs::new();
    inputs.print_result();
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_print_result_1_2() {
        let inputs = Inputs { a: 1, b: 2 };
        inputs.print_result();
    }

    #[test]
    fn test_print_result_10_2() {
        let inputs = Inputs { a: 10, b: 2 };
        inputs.print_result();
    }

    #[test]
    fn test_print_result_5_5() {
        let inputs = Inputs { a: 5, b: 5 };
        inputs.print_result();
    }
}
