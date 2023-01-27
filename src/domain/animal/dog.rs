use crate::domain::animal::human::Human;
use crate::json::json_parser::JsonDecoder;
use crate::json::lexical_analysis::decode_result::DecodeResult;

#[derive(Debug)]
pub struct Dog {
    name: String,
    age: isize,
    favorite_numbers: Vec<isize>,
    human: Human,
}

impl JsonDecoder<Dog> for Dog {
    fn parser(result: &DecodeResult) -> Dog {
        Dog {
            name: result.get("name"),
            age: result.get("age"),
            favorite_numbers: result.get("favoriteNumbers"),
            human: result.get("family"),
        }
    }
}
