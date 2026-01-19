/*
A very important concept in Rust, that will also help us understand other languages
in their lowest level behaviour, it the `move` semantics, so lets take a closer look
at this important feature.

Moving in rust imply that we will transfer ownership of a value from one variable to
a different one, i.e. if you move a value, the original variable that held it will no
longer have access to it, only the new one does.
*/

// Don't change this function
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
