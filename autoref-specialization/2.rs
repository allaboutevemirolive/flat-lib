// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

use std::fmt::{Display, Write};

pub trait DisplayToString {
    fn my_to_string(&self) -> String;
}

impl<T: Display> DisplayToString for &T {
    fn my_to_string(&self) -> String {
        println!("called blanket impl");

        let mut buf = String::new();
        buf.write_fmt(format_args!("{}", self)).unwrap();
        buf.shrink_to_fit();
        buf
    }
}

pub trait StringToString {
    fn my_to_string(&self) -> String;
}

// Capture initialize variable with string
impl StringToString for String {
    fn my_to_string(&self) -> String {
        println!("called specialed impl");

        self.clone()
    }
}

macro_rules! convert_to_strings {
    // The macro starts with a pattern that matches the input to the macro.
    // In this case, the input must be a comma-separated list of expressions.
    // The $(...),* syntax is used to capture multiple comma-separated expressions.
    // $e is a metavariable that represents each captured expression.
    ($($e:expr),*) => {
        // The macro expansion follows after the '=>' symbol.
        // Inside the expansion, we use square brackets to create an array.
        [$( // The $(...)* syntax is used again to repeat the following code for each captured expression.
            // Here, we take each captured expression and call the `my_to_string` method on it.
            // The `&$e` syntax borrows the expression because calling a method requires an immutable reference.
            // Assuming there is a method named `my_to_string` implemented for each type of expression.
            (&$e).my_to_string()
        ),*]
    };
}

fn main() {
    let owned_string = "hacks".to_owned();
    let strings = convert_to_strings![1, "&str", owned_string];
    println!("{:?}", strings);
}