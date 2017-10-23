fn main() {
    // We don't need to care about
    // the internal structure of NameLength
    // Instead, we can just call it's constructor
    let name_length = NameLength::new("John".to_string());

    // Prints "The name 'John' is '4' characters long"
    name_length.print();
}

struct NameLength {
    name: String,
    length: usize,
}

impl NameLength {
    // The user doesn't need to setup length
    // We do it for him!
    fn new(name: String) -> Self {
        NameLength {
            length: name.len(),
            name,
        }
    }

    fn print(&self) {
        println!(
            "The name '{}' is '{}' characters long",
            self.name,
            self.length
        );
    }
}
