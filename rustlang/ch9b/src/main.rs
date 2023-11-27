use std::fs::File;
use std::io::{self, Read};
// use std::io::Error;
use std::error::Error; // Import the Error trait

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}



fn main() -> Result<(), Box<dyn Error>> {
    let result = read_username_from_file()?;
    println!("These are the results: {:?}", result);

    Ok(())
}


