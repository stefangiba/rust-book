pub fn main() {
    if_expressions();
    repetition();
}

fn if_expressions() {
    let number = 7;

    if number != 0 {
        println!("number is something other than zero");
    }

    if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is higher than 5");
    }

    // Handling multiple conditions with if-else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn repetition() {
    loops();
}

fn loops() {
    // infinite loop
    // loop {
    //     println!("again!");
    // }

    breaking_out_of_loop();

    println!("");

    loop_labels();

    println!("");

    conditional_loops_with_while();

    println!("");

    looping_through_a_collection_with_while();

    println!("");

    looping_through_a_collection_with_for();

    println!("");

    looping_through_a_range_with_for();
}

fn breaking_out_of_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loops_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_through_a_collection_with_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn looping_through_a_collection_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn looping_through_a_range_with_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
