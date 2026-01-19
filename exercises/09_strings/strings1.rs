/*
Strings are a really interesting and annoying things in Rust, so we will stop
for a second and take a close look at them, because they're going to be one of our
friction points whilst using this language.

We have several types of string, to name a few, String, &str, CStr, &'static str...
but for now we're going to focus on String and &str.
 > &str:
    Our literals in the code (`"Hello, world!"`) defaults to this type.

    The reference (&) creates a reference with a known size at compile time, (though
    'str' itself is dynamically sized, but let's ignore that). This allows us to
    allocate it anywhere in our memory (stack, heap or even cache.) but keep the
    reference as a fat pointer (pointer + length) in our stack.

    (As a side note: it also implements deref coercion, which allows a reference
    to String (&String) to be converted and resolved to &str at compile time.)

 > String:
    We can use them through the API provided by the class String from Rust's std.

    On the other hand, this creates a growable, mutable (if we indicate so)
    vector with capacity, length and pointer that gets allocated in the heap and
    owned by the variable that holds it.
    This approach avoids the behavior of other languages such as Java, where a
    string is always immutable and has to be recreated to edit its content, leaving
    the original one for the Garbage Collector to handle.

    Note that slicing Strings will give us a &str.

Now we're all asleep, lets do something with this information.
*/

// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue"
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
