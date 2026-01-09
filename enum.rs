#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn new_circle(radius: f64) -> Self {
        Self::Circle(radius)
    }
}

fn main() {
    let circle: Shape = Shape::new_circle(5.0);
    println!("Circle: {:?}", circle);
}
