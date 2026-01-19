/*
It's time now to learn about Rust's enums. They're kinda weird at first, because
they're not really as much as our concept of an Enum is, but we'll learn about it.
*/

// TODO: Define a few types of messages as used below.
#[derive(Debug)]
enum Message {}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
