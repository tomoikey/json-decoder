extern crate core;

use crate::json_parser::JsonDecoder;

mod json_parser;
mod lexical_analysis;

fn main() {
    let dog_json = "{\"name\": \"taro\", \"age\": 8, \"favoriteNumbers\": [1, 2, 3, 4, 5], \"favoriteStrings\": [\"apple\", \"banana\"] }";
    let result = Dog::decode_from(dog_json);
    println!("{:?}", result);
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: usize,
    favorite_numbers: Vec<usize>,
    favorite_strings: Vec<String>,
}
