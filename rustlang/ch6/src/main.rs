// Enum for all 50 states
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

enum MyEnum {
    Number(Option<i32>),
    Name(Option<String>),
}

fn main() {
    let config_max = MyEnum::Number(Some(205));

    if let MyEnum::Number(val) = config_max {
        match val {
            Some(i) => println!("Number Value: {}", i),
            None => println!("Number No value!"),
        }
    } else if let MyEnum::Name(val) = config_max {
        match val {
            Some(i) => println!("Name Value: {}", i),
            None => println!("Name No value!"),
        }
    } else {
        println!("No match!");
    }
}



