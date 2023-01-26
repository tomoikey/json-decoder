use crate::json::json_parser::JsonDecoder;
use crate::json::lexical_analysis::decode_result::DecodeResult;

pub trait FromDecoderResult<T> {
    fn get(&self, key: &str) -> T;
}

impl FromDecoderResult<String> for DecodeResult {
    fn get(&self, key: &str) -> String {
        self.get_from_hash_map(key).as_str()
    }
}

impl FromDecoderResult<isize> for DecodeResult {
    fn get(&self, key: &str) -> isize {
        self.get_from_hash_map(key).as_number()
    }
}

impl FromDecoderResult<Vec<isize>> for DecodeResult {
    fn get(&self, key: &str) -> Vec<isize> {
        self.get_from_hash_map(key)
            .as_array()
            .into_iter()
            .map(|n| n.as_number())
            .collect()
    }
}

impl FromDecoderResult<Vec<String>> for DecodeResult {
    fn get(&self, key: &str) -> Vec<String> {
        self.get_from_hash_map(key)
            .as_array()
            .into_iter()
            .map(|n| n.as_str())
            .collect()
    }
}

impl<T> FromDecoderResult<T> for DecodeResult
where
    T: JsonDecoder<T>,
{
    fn get(&self, key: &str) -> T {
        T::parser(self.get_from_hash_map(key))
    }
}
