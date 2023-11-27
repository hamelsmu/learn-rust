#[derive(Debug)]
enum SpreadsheetCell {
    ID(i32),
    Float(f64),
    Text(String),
}

pub fn run(){
    let mut row = vec![
    SpreadsheetCell::ID(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),];


    println!("Hello from ex5.rs");
    println!("row: {:?}", row);
    println!("row: {:?}", row.pop());
    println!("row: {:?}", row);
}
