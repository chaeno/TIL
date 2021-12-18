#[macro_export]
macro_rules! vec1 {
    ( $( $x:expr), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    println!("{:?}", vec1![1, 2, 3]);
    Pancakes::hello_macro();
}
