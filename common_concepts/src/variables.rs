use std::io;

const _THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;

pub fn main() {
    variables();
    data_types();
}

fn variables() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let _spaces = spaces.len();
}

fn data_types() {
    // SCALAR TYPES

    // Integer
    /*
    | Length   | Signed | Unsigned |
    |----------|--------|----------|
    | 8-bit    | i8     | u8       |
    | 16-bit   | i16    | u16      |
    | 32-bit   | i32    | u32      |
    | 64-bit   | i64    | u64      |
    | 128-bit  | i128   | u128     |
    | arch     | isize  | usize    | -> depends on the architecture of the computer
    */

    // each signed variant can store numbers from -(2^(n-1)) to 2^(n-1) - 1
    // unsigned variants canm store numbers from 0 to 2^n - 1

    // type annotation is needed, otherwise the compiler won't know what implementation of
    // parse to use when trying to parse the contents of the string
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Floating-point
    // default is f64, as on modern CPUs it's more precise and rougly the same speed as f32.
    // floating point numbers are represented according to the IEEE-754 standard.
    // f32 uses single precision, while f64 uses double precision.
    // TODO: read about the differences between f32 and f64
    let x = 2.0; // f64
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference: i32 = (95.5 - 4.3) as i32;

    // multiplication
    let product = x * y;

    // division
    let _quotient = 56.7 / product;
    let _truncated = difference / sum;

    // remainder
    let _remainder = 43 % 5;

    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Character
    // 4 bytes in size
    // represents Unicode Scalar Value
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // COMPOUND TYPES

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;

    println!("The value of y is: {y}");

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // The tuple without anyu values has a special name, *unit*.

    // Array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // will contain 3 5s

    let _first = _a[0];
    let _second = _a[1];

    arrays_index_out_of_bounds_experiment();
}

fn arrays_index_out_of_bounds_experiment() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
