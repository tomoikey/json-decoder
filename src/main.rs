extern crate core;

use crate::json::json_parser::JsonDecoder;
use crate::json::lexical_analysis::lexical_analyzer::LexicalAnalysis;
use std::fs;
use std::time::Instant;

mod json;

fn main() {
    let start = Instant::now();
    for _ in 0..100000 {
        LexicalAnalysis::extract(
            " { \n   \"name\": \"taro\"\n,\"age\": 81 ,\n \"favoriteNumbers\" :  [  -1 , 2 , 3 , 4 , 5   ], \"family\": \n { \"name\": \"hoge\", \"age\": 8, \"array\": [  1 , 2 , 3  ] \n }\n }",
        );
    }
    let end = start.elapsed();
    let aaa = Dog::decode_from(
        " {    \"name\" : \"taro\" , \"age\": 81 , \"favoriteNumbers\" :   [  -1 , 2 , 3 , 4 , 5 ], \"family\": { \"name\": \"hoge\", \"age\": 8, \"array\": [1 , 2 , 3 ] } }",
    );
    println!("{:?}", aaa);
    println!(
        "{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: isize,
    favorite_numbers: Vec<isize>,
    human: Human,
}

#[derive(Debug)]
struct Human {
    age: isize,
    name: String,
    array: Vec<isize>,
}
