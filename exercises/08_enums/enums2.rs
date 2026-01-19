/*
We can create Enums that have different variants with different associated
types, such as anonymous structs, structs, primitive types, tuples,
no data at all... almost everything we can think of... but we're software
engineeers, please mind the KISS principle.

Also, it is important to know that Enum can have associated behaviours in
the same fashion as Struct do, so we can define `impl` blocks to define
functions associated with the struct itself.

Finally, notice that unlike in C++, we can use elements ahead of its definition,
i.e. we can use the Point struct at any given line, if we are sure we have
defined it somewhere within the file.
*/

// TODO: Define the different variants for Message used in main()
#[derive(Debug)]
enum Message {}

// TODO: implement an associated function `call()` for any instance
// of our Message enum
impl Message {}

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
