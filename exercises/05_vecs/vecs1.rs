/*
 We know arrays are the coolest guy around, but sometimes we need a more flexible approach. For that,
 we have Vec (Vectors) which are kinda similar to Java or Python lists.
*/

// TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    (a, v)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v); // This is such a cool line, investigate about deref coercion if you feel curious.
    }
}
