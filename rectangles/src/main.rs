#[derive(Debug)]

struct Rectangle {
    length: i32,
    width: i32
}

fn main() {
    let rec = Rectangle {
        length: 4,
        width: 5
    };

    println!("the area size of the rectangle is {}", area(&rec));
    println!("the rectangle dimensions are {:#?}", rec);
}

fn area(rectangle: &Rectangle) -> i32 {
    rectangle.length * rectangle.width
}
