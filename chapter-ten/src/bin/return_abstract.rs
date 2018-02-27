#![feature(conservative_impl_trait)]

trait Animal {
    fn do_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn do_sound(&self) {
        println!("Woof");
    }
}

fn main() {
    let animal = create_animal();
    animal.do_sound();
}

fn create_animal() -> impl Animal {
    Dog {}
}
