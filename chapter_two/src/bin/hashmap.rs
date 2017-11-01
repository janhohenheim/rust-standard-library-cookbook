use std::collections::HashMap;

fn main() {
    // The HashMap can map any hashable type to any other
    // The first type is called the "key"
    // and the second one the "value"
    let mut tv_ratings = HashMap::new();
    // Here, we are mapping &str to i32
    tv_ratings.insert("The IT Crowd", 8);
    tv_ratings.insert("13 Reasons Why", 7);
    tv_ratings.insert("House of Cards", 9);
    tv_ratings.insert("Stranger Things", 8);
    tv_ratings.insert("Breaking Bad", 10);

    // Does a key exist?
    let contains_tv_show = tv_ratings.contains_key("House of Cards");
    println!("Did we rate House of Cards? {}", contains_tv_show);
    let contains_tv_show = tv_ratings.contains_key("House");
    println!("Did we rate House? {}", contains_tv_show);

    // Access a value
    if let Some(rating) = tv_ratings.get("Breaking Bad") {
        println!("I rate Breaking Bad {} out of 10", rating);
    }

    // If we insert a value twice, we overwrite it
    let old_rating = tv_ratings.insert("13 Reasons Why", 9);
    if let Some(old_rating) = old_rating {
        println!("13 Reasons Why's old rating was {} out of 10", old_rating);
    }
    if let Some(rating) = tv_ratings.get("13 Reasons Why") {
        println!("But I changed my mind, it's now {} out of 10", rating);
    }


    // Iterating accesses all keys and values
    println!("All ratings:");
    for (key, value) in tv_ratings {
        println!("{}\t: {}", key, value);
    }
}
