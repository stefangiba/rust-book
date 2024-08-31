#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// everything within the `impl` block is associated with the `Rectangle` type
// all functions defined within an `impl` block are called `Associated Functions`
impl Rectangle {
    // methods need to a parameter named `self` of type `Self` (type alias for the type the `iompl` is for)
    // Rust allow us to abbreviate this with only the name self as the first parameter.
    //
    // - &self immutably borrows the current instance of the struct
    // - &mut self mutably borrows the current instance of the struct, useful for mutation
    // - self takes ownership of the instance of the struct, useful when turning self into smth else
    // and want to prevent the usage of the old instance
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// each struct is allowed to have multiple `impl` blocks. there's no reason to separate
// these methods into multiple `impl` blocks here, but it is valid syntax
impl Rectangle {
    // methods with the same name as the fields are usually used as getters / setters
    // these are useful because we can make the field private, and thus enable read-only
    // access to that field as part of the type's public API
    fn width(&self) -> bool {
        self.width > 0
    }
}

pub fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // dbg! takes and returns ownership of the expression's value
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // dbg! provides more debugging information compared to println! on a struct
    // instance that implements Debug, such as file name and line number
    dbg!(&rect1);

    println!("The rectangle is: {:?}", rect1);

    if rect1.width() {
        println!("The rectangle has a non-zero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 70,
    };

    if rect1.can_hold(&rect2) {
        println!("The second rectangle fits in the first one!");
    } else {
        println!("The second rectangle doesn't fit in the first one!");
    }

    // The `::` syntax is used for both associated functions and namespaces
    // created by modules
    let _square = Rectangle::square(25);
}
