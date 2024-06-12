
struct Sheep {
    name: &'static str,
    is_naked: bool
}


trait Animal {
    fn talk(&self) -> &'static str;

    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;



}


impl Animal for Sheep {
    fn talk(&self) -> &'static str {
        ""
    }

    fn new(name: &'static str) -> Self
}


fn main() {
}
