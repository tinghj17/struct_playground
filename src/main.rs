// standard struct  
struct User {
    active: bool,
    // using String here: we want these two fields to own the data 
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

// tuple structs 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs without any fields --> related to Chapter 10 trait 
struct AlwaysEqual;

// implementation for Rectangle struct 
impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let rect1 = Rectangle { 
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(3);
    println!(
        "The area of the square is {} square pixels.",
        sq.area());
}
