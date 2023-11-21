fn greet_world(){
    println!("Hello World");
    let southern_germany = "Grüß Gott!";         // <2>
    let japan = "ハロー・ワールド";                // <3>

    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!(":) {:?}", &region);
    }
}

fn main() {
    greet_world();
}
