#[macro_use]
extern crate serde_json;

use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    // A HashMap is the same as a JSON without any schema
    let mut key_value_map = HashMap::new();
    let stdin = io::stdin();
    println!("Please enter a key and a value or cancel with 'stop'");
    for input in stdin.lock().lines() {
        let input = input.expect("Failed to read line");
        if input == "stop" {
            break;
        }

        let key_value: Vec<_> = input.split_whitespace().collect();
        let key = key_value[0].to_string();
        let value = key_value[1].to_string();

        println!("Saving key-value pair: {} -> {}", key, value);
        // The json! macro lets us convert a value into its JSON representation
        key_value_map.insert(key, json!(value));
        println!("Enter another one or cancel with 'stop'");
    }
    // to_string_pretty returns a JSON with nicely readable whitespace
    let json =
        serde_json::to_string_pretty(&key_value_map).expect("Failed to convert HashMap into JSON");
    println!("Your input has been made into the following JSON:");
    println!("{}", json);
}
