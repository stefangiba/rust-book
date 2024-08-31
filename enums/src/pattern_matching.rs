#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn main() {
    println!(
        "Value in cents is: {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    println!("Rolled dice! Got: {:?}", roll_dice(9));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // comment any of the match branches and the code won't compile anymore
    // since the match expression is not exhaustive
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn roll_dice(dice_roll: u8) -> () {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        _ => reroll(),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(_num_spaces: u8) {}
fn reroll() {}
