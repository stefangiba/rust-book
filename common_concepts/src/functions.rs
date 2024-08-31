pub fn main() {
    // statement
    let _y = 6; // 6 is an expression that evaluates to 6

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
