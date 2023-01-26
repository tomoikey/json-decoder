use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub enum DecodeResult {
    Str(String),
    Number(isize),
    Array(Vec<Box<DecodeResult>>),
    Json(HashMap<String, Box<DecodeResult>>),
}

impl DecodeResult {
    pub fn get_from_hash_map(&self, key: &str) -> Self {
        use DecodeResult::*;
        match self {
            Json(json) => match json.get(key) {
                Some(value) => value.deref().clone(),
                None => panic!("key not found"),
            },
            _ => panic!("Jsonの場合のみ適用できます"),
        }
    }

    pub fn as_str(&self) -> String {
        use DecodeResult::*;
        match self {
            Str(value) => value.to_string(),
            _ => panic!("String型ではありません"),
        }
    }

    pub fn as_number(&self) -> isize {
        use DecodeResult::*;
        match self {
            Number(value) => value.clone(),
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_array(&self) -> Vec<DecodeResult> {
        use DecodeResult::*;
        match self {
            Array(value) => value.into_iter().map(|n| n.deref().clone()).collect(),
            _ => panic!("Array型ではありません"),
        }
    }
}
