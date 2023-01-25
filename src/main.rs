extern crate core;

use crate::json_parser::JsonDecoder;

mod json_parser;
mod lexical_analysis;

fn main() {
    let dog_json = "{\"name\": \"taro\", \"age\": 8}";
    let result = Dog::decode_from(dog_json);
    println!("{:?}", result);
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: usize,
}
