extern crate core;

use crate::json::json_parser::JsonDecoder;
use crate::json::lexical_analysis::lexical_analyzer::LexicalAnalysis;
use std::time::Instant;

mod json;

fn main() {
    let start = Instant::now();
    for _ in 0..1000000 {
        // let dog_json =
        // "{\"name\": \"taro\", \"age\": 8, \"favoriteNumbers\": [1, 2, 3, 4, 5], \"favoriteStrings\": [\"apple\", \"banana\"], \"family\": { \"age\": 1, \"name\": \"Tom\", \"array\": [1, 2, 4, 3] } }";
        // "{\"name\": \"taro\" }";
        LexicalAnalysis::extract(
            "{ \"name\": \"taro\" , \"age\" : 8, \"favoriteNumbers\" : [ 1 , 2 , 3 , 4 , 5 ] }",
        );
        // println!("{:?}", result.human);
    }
    let end = start.elapsed();
    println!(
        "{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: u8,
    favorite_numbers: Vec<u8>,
    favorite_strings: Vec<String>,
    human: Human,
}

#[derive(Debug)]
struct Human {
    age: u8,
    name: String,
    array: Vec<u8>,
}
