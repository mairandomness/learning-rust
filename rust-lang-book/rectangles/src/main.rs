// Code without structs or tuples
// fn main() {
//     let width1 = 30;
//     let heigth1 = 50;

//     println!("The area of the rectangle is {} square pixels.", area(width1, heigth1));
// }

// fn area (width: u32, height: u32) -> u32 {
//     width * height
// }




//code with tuples
// fn main() {
//     let ret = (30, 50);

//     println!("The area of the rectangle is {} square pixels.", area(ret));
// }

// fn area (dimensions: (u32, u32)) -> u32 {
//      dimensions.0 * dimensions.1
// }




//code with structs but no methods
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {width: 30, height: 50};

//     println!("The area of the rectangle is {} square pixels.", area(&rect1));
//     println!("The perimeter of the rectangle is {} pixels.", perimeter(&rect1));
//     println!("rect1 is {:?}", rect1);
// }

// fn area (rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn perimeter (rectangle: &Rectangle) -> u32 {
//     2 * (rectangle.width + rectangle.height)
// }




//code with structs and methods!!
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //there can be multiple impl blocks for the same struct, this programm only has one though.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //this is an associeted function (doesn't take a self as a parameter)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    //the function square exists in the namespace Rectangle
    //that's why we cal it using ::
    let sqr1 = Rectangle::square(4);


    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The perimeter of the rectangle is {} pixels.", rect1.perimeter());
    println!("rect1 is {:?}", rect1);
    println!("This is sqr1 {:?}", sqr1);
    println!("");
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

