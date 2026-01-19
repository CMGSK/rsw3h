/*
Now one of my favourite parts of Rust. Options. What is an Option? Well, its
either a value or the lack of it. Now, you can tell me this might be then a
similar thing to a nullable variable in Java, C# or the like, but not quite.
Let's see:

Rust has no `null`. Instead, we have Option values which translates to an enum
with two variants: Some(T) and None.
None is what in other cases we would understand as a null value, however, since
its not a null and its an enum variant, we cannot force the compiler to interpret
it as the type we're supposedly handling.
If the value exists, it is wrapped inside a Some(T) variant where T is a generic
that can hold any know type in out application (including custom structs)

In any case, what we really have is an enum type Option. To unwrap or check
the possible outcomes of any given Option, we have several functions that can be
used, but we're going to mainly use three:
    > unwrap functions:
        Unwrap functions will try to unwrap (duh) the value into the type we
        were supposed to get. Some of them will allow us to define a fallback
        value, and some others will panic our application when they find None.
    > let patterns:
        We can define `let Some(foo) else {}` or `if let Some(foo) {}` to try
        initialize a `foo` variable with the correct type or else...
    > pattern matching:
        We can use a match expression.

Note that since this is part of Rust's std, we can create Some(_) and None
by defining it directly inline.
*/

/*
Out mom lets us eat ice cream sometimes. She's very strict, so she will only allow
us to eat either a single scoop from 10AM to 12AM, or two from 17PM to 19PM.
However, sometimes we wake up at 3AM and very carefully go to the fridge and eat
three hole scoops because we know she's asleep. Keep it secret though.
*/
// TODO: use pattern matching to check how many scopps of ice cream we can eat
// depending on the hour of the day (24h format)
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Fix this test. How do you get the value contained in the
    // option? We already did pattern matching, try other way now.
    #[test]
    fn raw_value() {
        let ice_creams = maybe_ice_cream(12);
        assert_eq!(ice_creams, 5);
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), None);
        assert_eq!(maybe_ice_cream(3), Some(3));
        assert_eq!(maybe_ice_cream(6), None);
        assert_eq!(maybe_ice_cream(11), Some(1));
        assert_eq!(maybe_ice_cream(12), Some(1));
        assert_eq!(maybe_ice_cream(17), Some(2));
        assert_eq!(maybe_ice_cream(18), Some(2));
        assert_eq!(maybe_ice_cream(24), None);
    }
}
