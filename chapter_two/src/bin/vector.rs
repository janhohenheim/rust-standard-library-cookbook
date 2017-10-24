fn main() {
    // Create a vector with some elements
    let fruits = vec!["apple", "tomato", "pear"];
    // A vector cannot be directly printed
    // But we can debug-print it
    println!("Fruits: {:?}", fruits);

    // Create an empty vector and fill it
    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("tomato");
    fruits.push("pear");
    println!("Fruits: {:?}", fruits);
}
