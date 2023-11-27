use rand::Rng;
use std::collections::HashMap;
use std::{cmp::Ordering, io};

mod foo;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number is: {}", secret_number);
    foo::bar::hamel();
}
