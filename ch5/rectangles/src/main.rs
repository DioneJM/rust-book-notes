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

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.area() > other_rect.area()
    }
}
fn main() {
    println!("Hello, world!");

    let rect: Rectangle = Rectangle::new(10, 32);
    println!("Rectangle area: {}", rect.area());

    let big_rect: Rectangle = Rectangle::new(100, 90);
    let smaller_rect: Rectangle = Rectangle::new(99, 89);
    println!("Can hold: {}", big_rect.can_hold(&smaller_rect));
}
