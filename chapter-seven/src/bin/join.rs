extern crate rayon;

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
}

fn main() {
    let rect = Rectangle {
        height: 30,
        width: 20,
    };
    let (area, perimeter) = rayon::join(|| rect.area(), || rect.perimeter());
    println!("{:?}", rect);
    println!("area: {}", area);
    println!("perimeter: {}", perimeter);
}
