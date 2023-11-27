use std::collections::HashMap;

pub fn run(){
    let mut cars = HashMap::new();
    // let val = "hi hamel".to_string();
    
    cars.insert("Red", String::from("hi hamel"));
    println!("cars: {:?}", cars);
    // println!("val: {:?}", val);

    let name = "Hamel";
    let val2 = "30";
    cars.insert(name, "30".to_string());
    println!("cars: {:?}", cars);
}