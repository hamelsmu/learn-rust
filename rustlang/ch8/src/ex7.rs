pub fn run(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {}", s);
    for i in s.chars() {
        println!("i: {}", i);
    }

    for i in s.split("-") {
        println!("i: {}", i);
    }
    let foo = &s[0..3];
    println!("s slice: {}", foo);
}