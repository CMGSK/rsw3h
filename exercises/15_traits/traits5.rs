/*
For the last part of the workshop, I want to make a brief introduction to traits. 
Although this is something we will see more in depth in a intermediate level workshop, 
traits are things you will encounter sometimes, and we need to understand the basis.

With that said, we're not going to learn how to define and implement custom traits 
(although you may understand how just by reading the following lines) but instead what
we can do with them in case we need them.

For example, they're useful things to always receive or return types that can be 
converted to string, or sets of different structs that implement certain functionalities. 
They're also often used along with generics, so you will see them in many libraries 
inline documentation.
*/

// Do not modify the following lines
trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
struct OtherStruct;

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// Here's your working area:

// TODO: Fix the compiler error by only changing the signature of this function.
fn some_func(item: ???) -> bool {
    item.some_function() && item.other_function()
}

// TODO: Fix the compiler error by only changing the signature of this function
// keeping item the same type as previously
fn other_func(item: ???) {
    item
}

fn main() {
    // Don't mind this.
    _ = other_func(SomeStruct);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
