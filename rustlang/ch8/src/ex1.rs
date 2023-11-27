pub fn run(){
    let mut v = vec![1, 2, 3, 4, 5];
    let first = get_first(&v);
    v.push(6);
    println!("The first element is: {first}");
}

fn get_first(v: &Vec<i32>) -> i32 {
    v[0]
}