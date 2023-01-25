use crate::json::from_decoder_result::FromDecoderResult;
use crate::json::lexical_analysis::{DecodeResult, LexicalAnalysis};
use crate::Dog;

pub trait JsonDecoder<T> {
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
            favorite_numbers: result.get("favoriteNumbers"),
            favorite_strings: result.get("favoriteStrings"),
        }
    }
}
