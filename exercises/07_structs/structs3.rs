/*
This is a dense topic. Let's take a look at the "not-really-OOP" in Rust. If we
have some C++ knowledge, this will ring a bell to you. In Rust we have an approach
based on defining `struct`s, which contain data, and `impl`s, which contain logic.

We also have other things we can define such as traits heavily related to OOP, but
we will keep it aside for the intermediate lessons.

Structs are custom types that groups related data. Impls links logic to our structs.

Before starting with it, you will notice a sort-of decorator on top of our struct.
This decorator is called attribute macro in Rust terminology, and it just modify how
the compiler process it by automatically implementing some behaviours (traits) for
it without the need of writing the code ourselves.
In this case, Debug will allow us to print the struct for debugging purposes in a
human-readable fashion.
*/

//TODO: Add a `recipient_country` String type to the Package struct
#[derive(Debug)]
struct Package {
    sender_country: String,
    weight_in_grams: u32,
}

// TODO: Add the correct return type to all the function signatures
// TODO: Add the additional field of the struct to the constructor
impl Package {
    fn new(origin: String, recipient_country: String, weight_in_grams: u32) {
        /*
        Since there's not a strict definition of 'constructor' in rust, the `new` function
        is usually defined to serve that purpose. To explicitly declare a struct, we can use
        the name of the struct and a pair of curly braces to define its associated values as
        long as its in our 'scope'
        */
        Self {
            sender_country: origin,
            weight_in_grams,
        }
    }

    // TODO: Read the tests that use this method to find out when a package
    // is considered international.
    fn is_international(&self) {}

    // TODO: Calculate the package's fees (grams * cents_per_gram)
    fn get_fees(&self, cents_per_gram: u32) {}
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
