/*
Remember the logic about how &str is non-mutable and often used for read only and slicing?
Let's use our logic to guess which of them we need to use
*/

// Calls of this function should be replaced with a call of one of the other two.
fn placeholder() {
    panic!()
}

fn is_slice(arg: &str) {
    println!("{arg}");
}

fn is_string(arg: String) {
    println!("{arg}");
}

// TODO: Your task is to replace `placeholder(…)` with either `is_slice(…)`
// or `is_string(…)` depending on what you think each value is.
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());

    /*
    This is slicing. Syntax is String[range].
    Ranges in rust are:
        0..5 -> inclusive, exclusive
        0..=5 -> inclusive, inclusive
        0.. -> N to end
        ..5 -> start to N exlusive
        ..=5 -> start to N inclusive
    */
    placeholder(&String::from("abc")[0..1]);
}
