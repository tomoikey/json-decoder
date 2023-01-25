extern crate core;

use crate::lexical_analysis::{DecodeResult, LexicalAnalysis};

mod lexical_analysis;

fn main() {
    let dog_json = "{\"name\": \"taro\", \"age\": 8}";
    let result = Dog::decode_from(dog_json);
    println!("{:?}", result);
}

trait JsonDecoder<T> {
    fn parser(result: DecodeResult) -> T;

    fn decode_from(json: &str) -> T {
        Self::parser(LexicalAnalysis::extract(json).unwrap().1)
    }
}

impl JsonDecoder<Dog> for Dog {
    fn parser(result: DecodeResult) -> Dog {
        Dog {
            name: result.get("name").as_str(),
            age: result.get("age").as_number() as u8,
        }
    }
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: u8,
}
