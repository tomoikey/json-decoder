extern crate core;

use crate::json::json_parser::JsonDecoder;

mod json;

fn main() {
    let dog_json = "{\"name\": \"taro\", \"age\": 8, \"favoriteNumbers\": [1, 2, 3, 4, 5], \"favoriteStrings\": [\"apple\", \"banana\"], \"family\": { \"age\": 1, \"name\": \"Tom\", \"array\": [1, 2, 4, 3] } }";
    let result = Dog::decode_from(dog_json);
    println!("{:?}", result);
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: usize,
    favorite_numbers: Vec<usize>,
    favorite_strings: Vec<String>,
    human: Human,
}

#[derive(Debug)]
struct Human {
    age: usize,
    name: String,
    array: Vec<usize>,
}
