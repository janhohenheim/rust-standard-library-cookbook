fn main() {
    // Create a vector with some elements
    let fruits = vec!["apple", "tomato", "pear"];
    // A vector cannot be directly printed
    // But we can debug-print it
    println!("fruits: {:?}", fruits);

    // Create an empty vector and fill it
    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("tomato");
    fruits.push("pear");
    println!("fruits: {:?}", fruits);

    // Initialize the vector with a value
    // Here, we fill our vector with five zeroes
    let bunch_of_zeroes = vec![0; 5];
    println!("bunch_of_zeroes: {:?}", bunch_of_zeroes);

    
}
