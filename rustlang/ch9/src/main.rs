use std::fs::File;
use std::io::{self, Read};
use std::time::SystemTime;
use std::error::Error;

// fn main2() {
//     match greeting_file_result {
//         Ok(file) => {
//             let md = file.metadata();
//             match md {
//                 Ok(md) => {
//                     match md.created() {
//                         Ok(time) => println!("Create time: {:?}", time),
//                         Err(error) => println!("Failed to read create time: {:?}", error),
//                     }
//                 },
//                 Err(error) => println!("Failed to read create time: {:?}", error),
//             }
//         },
//         Err(error) => println!("Failed to open file: {:?}", error),
//     }
// }

fn main() -> Result<(), io::Error> {
    run()
}

fn run() -> Result<(), io::Error> {
    let username = read_username_from_file()?;
    println!("username: {:?}", username);
    let last_char = last_char_of_first_line(&username);
    match last_char {
        Some(c) => println!("last_char: {:?}", c),
        None => println!("last_char: None"),
    }
    Ok(())
    // let result = last_char_of_first_line(&username)?;
    // println!("result: {:?}", result);
    // Ok(result)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last() //next()? will raise None if text.lines() is empty which is why we need <Option>.
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
