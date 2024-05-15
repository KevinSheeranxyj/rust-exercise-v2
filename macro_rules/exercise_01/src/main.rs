use crate::designator_use_case::designator_test;

mod designator_use_case;
macro_rules! say_hello {
    () => {println!("hello")};
}

fn main() {

    say_hello!();
    designator_test();
}