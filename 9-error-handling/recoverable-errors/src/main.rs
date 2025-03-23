use std::fs::File;
use std::{fs, io};
use std::error::Error;
use std::io::{ErrorKind, Read};
use log::error;


fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match &greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}")
    // };
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}")
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");
    //         }
    //     }
    // };


    // alternate way to write the above code

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}")
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}")
    //     }
    // });

    // shortcut for panic Error

    // let greeting_file = File::open("hello.txt").unwrap();

    // a better way to provide context of the operation
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // cannot use ? here
    // incompatible return type with main
    // since main returns ()
    // and ? is used for early and expects return type of main()
    // to be Result but it isn't
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}


// propagating errors
// longer way
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// short way
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// shorter way
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// shortest way
fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// using ? operator with Option Type
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}