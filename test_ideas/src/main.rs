extern crate arraylist;

use std::fmt;
use arraylist::arl::ArrayList;

fn main() {
    #[derive(Debug, Clone, PartialEq)]
    struct Person<'a> {
        name: &'a str,
        age: u32,
    }

    // To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
    impl fmt::Display for Person<'_> {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "Person: ({} {})", self.name, self.age)
        }
    }

    let array = ArrayList::<Person>::default();
    array.push(Person { name: "boris", age: 23, });
    array.push(Person { name: "Tom", age: 34, });
    array.push(Person { name: "Harry", age: 54, });
    array.push(Person { name: "Francis", age: 34, });
    array.push(Person { name: "Phil", age: 23, });
    array.push(Person { name: "Sam", age: 67, });
    array.push(Person { name: "Ted", age: 45, });
    array.push(Person { name: "Fred", age: 14, });


    array.for_each(|a| {
        println!("{}",a)
    });

}
