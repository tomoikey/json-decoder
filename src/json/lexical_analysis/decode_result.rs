use crate::json::json_parser::JsonDecoder;
use std::collections::HashMap;
use std::ops::Deref;
use DecodeResult::*;

#[derive(Debug, PartialEq, Clone)]
pub enum DecodeResult {
    Str(String),
    Number(isize),
    Array(Vec<Box<DecodeResult>>),
    Json(HashMap<String, Box<DecodeResult>>),
}

impl DecodeResult {
    fn get_from_hash_map(&self, key: &str) -> &Self {
        match self {
            Json(json) => match json.get(key) {
                Some(value) => value.deref(),
                None => panic!("key not found"),
            },
            _ => panic!("Jsonの場合のみ適用できます"),
        }
    }

    pub fn get<T>(&self, key: &str) -> T
    where
        T: JsonDecoder<T>,
    {
        T::parser(self.get_from_hash_map(key))
    }

    pub fn as_str(&self) -> String {
        match self {
            Str(value) => value.to_string(),
            _ => panic!("String型ではありません"),
        }
    }

    pub fn as_number(&self) -> isize {
        match *self {
            Number(value) => value,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_array(&self) -> Vec<DecodeResult> {
        match self {
            Array(value) => value.into_iter().map(|n| n.deref().clone()).collect(),
            _ => panic!("Array型ではありません"),
        }
    }
}
