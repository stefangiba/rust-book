pub fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    // if you have a mutable reference to a value, you can have no other references to that value
    // DOES NOT COMPILE
    // let ref1 = &mut s1;
    // let ref2 = &mut s1; // INVALID
    // println!("{}, {}", ref1, ref2);

    // DOES NOT COMPILE
    // let ref1 = &s1;
    // let ref2 = &s1; // both are valid
    // let ref3 = &mut s1; // INVALID
    // println!("{}, {}, and {}", ref1, ref2, ref3);

    // a reference's scope starts from where it is introduced and continues through the
    // last time the reference is used
    // the following code is valid because the last usage of the immutable references
    // occurs before the mutable reference is introduced
    let ref1 = &s1;
    let ref2 = &s1;
    println!("{}, {}", ref1, ref2);
    // scope of ref1 and ref2 ends here

    let ref3 = &mut s1; // this is valid
    println!("{}", ref3);

    let _valid_str = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope, but because it does not have ownership of what it refers to, it si not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// DOES NOT COMPILE
// fn dangle() -> &String {
//     // returns a reference to a String
//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // here s goes out of scope, and is dropped. Its memory is freed. Danger!

fn no_dangle() -> String {
    String::from("hello")
}
