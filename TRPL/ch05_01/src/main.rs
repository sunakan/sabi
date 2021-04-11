#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    // ---------------------引数２つ版
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // ---------------------リファクタ1
    let rect1 = (width1, height1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // ---------------------リファクタ2
    let rect2 = Rectangle {
        width: width1,
        height: height1,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);

    // ---------------------リファクタ3
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
