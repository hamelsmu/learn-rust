const DAYS : [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
              "eighth", "ninth", "tenth", "eleventh", "twelfth"];
const GIFTS : [&str; 12] = ["A partridge in a pear tree", "Two turtle doves", "Three French hens",
               "Four calling birds", "Five gold rings", "Six geese a-laying",
               "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing",
               "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
             
fn intro(x: usize) -> String {
    format!("On the {} day of Christmas my true love gave to me", DAYS[x])
}

fn fib(x: usize) -> usize {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fib(x-1) + fib(x-2)
    }
}

fn farenheit_to_celsius(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}

fn main() {
    for i in 0..12 {
        println!("{}", intro(i));
        for j in (0..i+1).rev() {
            println!("{}", GIFTS[j]);
        }
        println!("");
    }
    let ans = fib(32);
    println!("fib(32) = {}", ans);

    let ans = farenheit_to_celsius(98.6);
    println!("98.6 F = {} C", ans);    
}
