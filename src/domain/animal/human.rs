use crate::json::json_parser::JsonDecoder;
use crate::json::lexical_analysis::decode_result::DecodeResult;

#[derive(Debug)]
pub struct Human {
    age: isize,
    name: String,
    array: Vec<isize>,
}

impl JsonDecoder<Human> for Human {
    fn parser(result: &DecodeResult) -> Human {
        Human {
            name: result.get("name"),
            age: result.get("age"),
            array: result.get("array"),
        }
    }
}
