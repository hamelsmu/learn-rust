mod ex1;
mod ex2;
mod ex3; mod ex4; mod ex5; mod ex6; mod ex7;
mod ex8;
mod ex9;
mod ex10;

fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(43);
    println!("Hello, world! {:?}", v);

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(v) => println!("The third element is {}", v),
    //     None => println!("There is no third element."),
    // }

    // let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);
    println!("The third element is {:?}", third);

    ex2::run();
    ex3::run();
    ex4::run();
    ex5::run();
    ex6::run();
    ex7::run();
    ex8::run();
    ex9::run();
    ex10::run();
}   
