pub fn run(){
    let mut s = "Hello".to_string();
    s.push('H');
    s.push_str("ello");

    println!("Hello from ex6.rs");
    println!("s: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    
}