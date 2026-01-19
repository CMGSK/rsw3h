/*
A very important thing to take into consideration involving ownership and
borrowing, lets check a fundamental Rust rule we will have to always take
into consideration:

We can have N non-mutable references OR a single mutable reference at a time,
and never mix them.

Now, this doesn't mean we cannot define them within the same block of code,
but that they will get invalidated, because the internal value is borrowed.

This is a fundamental in Rust. When we move, we transfer the ownership, thus
the original variable becomes invalid and can never be used again.
However, we can instead temporarily grant access with a borrow (&mut) i.e.
the original variable still owns the data, but it's locked while the borrow
exists.

Since we cannot have more than a single mut reference (borrow) at a time,
but we can define them, at any given time, the only valid one will be the
last one defined.
*/
fn main() {}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
