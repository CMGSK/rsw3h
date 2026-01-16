/*
A quick view on some primitives. Lets not stop with integers, bools and so, but
I think this syntax is something to know about.
*/

// TODO: Create an array called `a` with at least 100 elements in it.
fn main() {
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
