trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}
fn test(shape: dyn Animal) {
    shape.speak();
}
