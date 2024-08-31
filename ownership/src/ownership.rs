pub fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!"); // appends a literal to a String

    println!("{s}"); // This will print `Hello, world!`

    let x = 5; // bind value 5 to x
    let y = x; // make a copy of the value in x and bind it to y
    println!("y is {y}");

    let s1 = String::from("hello"); // allocating a new String s1
    let s2 = s1; // move the ownership of the heap allocated value bound to s1 to s2

    println!("{}, world!", s2);

    takes_ownership(s2); // s2's value moves into the function
                         // it is no longer valid here

    // println!("{}", s2); // this will throw an error because s2 is no longer valid

    let x = 5;

    // x would move into the function, but i32 implements Copy, so it is ok to use x afterwards
    makes_copy(x);

    println!("x is {x}");

    let _s3 = gives_ownership(); // gives_ownership moves its return value into s3
    let s4 = String::from("hello"); // s4 comes into scope
    let _s5 = takes_and_gives_back(s4); // s4 is moved into takes_and_gives_back, which also moves its return value into s5
} // _s5 goes out of scope and is dropped, s4 was moved, so nothing happens
  // _s3 goes out of scope and is dropped

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // here, some_integer goes out of scope. Nothing special happens

// will move its return value into the function that calls it
fn gives_ownership() -> String {
    String::from("yours") // this is returned and moves out to the calling function
}

// takes ownership of a String and returns it back
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // this is returned and moves out to the calling function
}
