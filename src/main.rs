extern crate core;

use crate::domain::animal::dog::Dog;
use crate::json::json_parser::JsonDecoder;
use crate::json::lexical_analysis::lexical_analyzer::LexicalAnalysis;
use std::fs;
use std::time::Instant;

mod domain;
mod json;

fn main() {
    let moji = "a".repeat(100);
    let json = String::from("{") + format!("{}: 1,", moji).as_str() + "\"name\": \"taro\"\n,\"age\": 81 ,\n \"favoriteNumbers\" :  [  -1 , 2  ], \"family\": \n { \"name\": \"hoge\", \"age\": 8, \"array\": [   ] \n }";
    let json = json.as_str();
    let start = Instant::now();
    for _ in 0..100000000 {
        LexicalAnalysis::extract(json);
    }
    // let dog = Dog::decode_from(json);
    let end = start.elapsed();
    println!(
        "{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
