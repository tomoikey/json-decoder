use crate::json::lexical_analysis::DecodeResult;

pub trait FromDecoderResult<T> {
    fn get(&self, key: &str) -> T;
}

impl FromDecoderResult<String> for DecodeResult {
    fn get(&self, key: &str) -> String {
        self.get_from_hash_map(key).as_str()
    }
}

impl FromDecoderResult<usize> for DecodeResult {
    fn get(&self, key: &str) -> usize {
        self.get_from_hash_map(key).as_number()
    }
}

impl FromDecoderResult<Vec<usize>> for DecodeResult {
    fn get(&self, key: &str) -> Vec<usize> {
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
