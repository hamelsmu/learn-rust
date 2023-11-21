use std::any::type_name;


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
   return &s[..]; // if no space found, return length of string
}

fn type_name_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let s = String::from("Thequick brown world");
    let t = "Thequick brown world";

    for (i, item) in t.split_whitespace().enumerate() {
        println!("{} {}", i, item);
    }

    println!("First word: {s}");
    println!("Type of s: {}", type_name_of(&s));
    println!("Type of t: {}", type_name_of(&t));

    let word: String = first_word(&s).to_string();
    println!("First word: {}", word);
}