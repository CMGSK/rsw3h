/*
We have shadowing in Rust, just like we do have it in python and other languages.
*/

// TODO: Fix the compiler error by changing the line below without renaming the variable.
fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    number = 3;
    println!("Number plus two is: {}", number + 2);
}
