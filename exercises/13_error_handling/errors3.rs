/*
Similarly to Options to hold possible null values, we have Results to hold possible
Errors. It is the same concept, but changing Some(T) and None with Ok(T) and Err(E),
where E is any type that defines our Error. It can be std::Error, custom structs,
primitives, or even a unit type (`Err(())`) which is the common way of defining
void-like operations.

Also, let me introduce the `?` syntax. This can also be used with Option. When we
make a call to a function that returns either Option or Result, we can use ? to
automatically unwrap the value inline, or else, short-circuit the function returning
None or Err(E) depening on our function signature.
*/

use std::num::ParseIntError;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing function's body and signature.
fn try_spend(user_input: &str) {
    let mut coins = 50;

    // Don't change this line.
    let cost = total_cost(user_input)?;

    if cost > coins {
        println!("You can't afford that many!");
    } else {
        coins -= cost;
        println!("You now have {coins} coins.");
    }
}
fn main() {
    try_spend("8");
    try_spend("80");
    try_spend("eight");
}
