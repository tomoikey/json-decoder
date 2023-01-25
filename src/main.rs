use crate::lexical_analysis::LexicalAnalysis;

mod lexical_analysis;

fn main() {
    let json_in_json =
        "{ \"hello\": 40, \"json\": { \"age\": 1, \"name\": \"Tom\", \"array\": [1, 2, 4, \"3\"] }}";

    println!("raw json -> {}\n", json_in_json);
    let result = LexicalAnalysis::extract(json_in_json).unwrap().1;
    let aaaa = result.get("hello");
    println!("{:?}", result);
}
