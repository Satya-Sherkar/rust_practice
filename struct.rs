struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated functions
    fn new(new_width: u32, new_height: u32) -> Self {
        Self {
            width: new_width,
            height: new_height,
        }
    }
    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect_one = Rectangle::new(10,5);

    let area = rect_one.area()
    println!("Area of rectangle is: {area}");
}
