use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello3.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("hello4.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello5.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v4() -> Result<String, io::Error> {
    Ok(fs::read_to_string("hello6.txt")?)
}

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    match read_username_from_file_v4() {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("파일에서 값을 읽어오지 못했습니다. v4 {:?}", e),
    }

    match read_username_from_file_v3() {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("파일에서 값을 읽어오지 못했습니다. v3 {:?}", e),
    }

    match read_username_from_file_v2() {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("파일에서 값을 읽어오지 못했습니다. v2 {:?}", e),
    }

    match read_username_from_file() {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("파일에서 값을 읽어오지 못했습니다. {:?}", e),
    }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일을 생성하지 못했습니다. {:?}", e),
            },
            other_error => panic!("파일을 열지 못했습니다: {:?}", other_error),
        },
    };

    // let f = File::open("hello1.txt").unwrap();
    let f = File::open("hello2.txt").expect("파일을 열 수 없습니다.");
}
