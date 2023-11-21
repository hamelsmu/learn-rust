#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn sq(sz: u32) -> Self{
        Self {
            width: sz,
            height: sz,
        }
    }
}

fn main() {
    let scale = 2;
    let r = Rectangle {
        width: 30 * scale, 
        height: 50
    };

    let r2 = Rectangle {
        width: 10,
        height: 40
    };
    let r3 = Rectangle::sq(10);

    println!(
        "The area of the rectangle is {} square pixels with dim {}x{}. Can it hold r2 {}x{}: {}",
        r.area(),
        r.width,
        r.height,
        r2.width,
        r2.height,
        r.can_hold(&r2)
    );

    println!("r3: {:#?}", r3);
}

// fn area(r: &Rectangle) -> u32 {
//     r.width * r.height
// }