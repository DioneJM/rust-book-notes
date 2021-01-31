struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }
}
fn main() {
    println!("Hello, world!");

    let rect: Rectangle = Rectangle::new(10, 32);
    println!("Rectangle area: {}", rect.area());
}
