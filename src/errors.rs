use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn read_user_from_file() -> Result<String, io::Error> {
    let user_file_result = File::open("hello.txt");

    let mut username_file = match user_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn q_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn s_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() -> Result<(), Box<dyn Error>>{
    //panic!("crash and burn");

    let v = vec![1, 2, 3];

    //&v[10];

    let f = File::open("hello.txt");

    let r = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("error creating file {e:?}"),
            },
            _ => {
                panic!("error opening file {e:?}");
            }
        },
    };

    let f = File::open("abc.txt").expect("error opening file");

    let gf = File::open("hello.txt")?;

    Ok(())
}
