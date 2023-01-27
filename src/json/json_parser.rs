use crate::json::lexical_analysis::decode_result::DecodeResult;
use crate::json::lexical_analysis::lexical_analyzer::LexicalAnalysis;

pub trait JsonDecoder<T> {
    fn parser(result: &DecodeResult) -> T;

    fn decode_from(json: &str) -> T {
        let result = LexicalAnalysis::extract(json).unwrap().1;
        Self::parser(&result)
    }
}

impl JsonDecoder<isize> for isize {
    fn parser(result: &DecodeResult) -> isize {
        result.as_isize()
    }
}

impl JsonDecoder<String> for String {
    fn parser(result: &DecodeResult) -> String {
        result.as_str()
    }
}

impl<T> JsonDecoder<Vec<T>> for Vec<T>
where
    T: JsonDecoder<T>,
{
    fn parser(result: &DecodeResult) -> Vec<T> {
        result.as_array().iter().map(|n| T::parser(n)).collect()
    }
}
