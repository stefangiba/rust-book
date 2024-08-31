// slices allow referencing a contiguous sequence of elements in a collection
// rather than the whole collection. a slice is a kidnd of reference, so it
// does not have ownership

pub fn main() {
    string_slices();
    other_slices();
}

fn string_slices() {
    let s = String::from("hello world");

    // works on slices of Strings, wether partial or whole
    let _word = first_word(&s[0..6]);
    let _word = first_word(&s[..]);
    // also works on references to Strings, which are equivalent
    // to whole slices of Strings
    let word = first_word(&s);

    // DOES NOT COMPILE
    // s.clear(); // this empties the String, making it equal to ""

    println!("the first word is: {}", word);

    let s = "Hello, world!";
    // works on slices of string literals, wether partial or whole
    let _word = first_word(&s[0..6]);
    let _word = first_word(&s[..]);
    // because string literals are string slices already, this also
    // works without the slice syntax
    let _word = first_word(s);
}

// passing in a parameter of type `&str` allows us to use the same function
// for both `&String` and `&str` types
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // `.iter()` returns an iterator over the collection
    // `.enumerate()` wraps the result of `.iter()` and returns each element as a tuple
    for (idx, &item) in bytes.iter().enumerate() {
        // compare current item with the space byte literal
        if item == b' ' {
            return &s[..idx];
        }
    }

    &s[..]
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
