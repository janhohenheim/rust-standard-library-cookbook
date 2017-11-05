std::collections::HashSet;

fn main() {
    // Most of the interface of HashSet 
    // is the same as HashMap, just without
    // the methods that handle values
    let mut books = HashSet::new();
    books.insert("Harry Potter and the Philosopher's Stone");
    books.insert("The Name of the Wind");
    books.insert("A Game of Thrones");

    // A HashSet will ignore duplicate entries
    // but will return if an entry is new or not
    let is_new = books.insert("The Lies of Locke Lamora");
    if is_new {
        println!("We've just added a new book!");
    }

    let is_new = books.insert("A Game of Thrones");
    if is_new {
        println!("Sorry, we already had that book in store");
    }
}