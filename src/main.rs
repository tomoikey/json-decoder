extern crate core;

use crate::lexical_analysis::{DecodeResult, FromDecoderResult, LexicalAnalysis};

mod lexical_analysis;

fn main() {
    let dog_json = "{\"name\": \"taro\", \"age\": 8}";
    let result = Dog::decode_from(dog_json);
    println!("{:?}", result);
}

trait JsonDecoder<T> {
    fn parser(result: DecodeResult) -> T;

    fn decode_from(json: &str) -> T {
        let result = LexicalAnalysis::extract(json).unwrap().1;
        Self::parser(result)
    }
}

impl JsonDecoder<Dog> for Dog {
    fn parser(result: DecodeResult) -> Dog {
        Dog {
            name: result.get("name"),
            age: result.get("age"),
        }
    }
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: usize,
}
