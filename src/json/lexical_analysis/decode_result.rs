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

    pub fn as_i8(&self) -> i8 {
        match *self {
            Number(value) => value as i8,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_i16(&self) -> i16 {
        match *self {
            Number(value) => value as i16,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_i32(&self) -> i32 {
        match *self {
            Number(value) => value as i32,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_i64(&self) -> i64 {
        match *self {
            Number(value) => value as i64,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_i128(&self) -> i128 {
        match *self {
            Number(value) => value as i128,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_isize(&self) -> isize {
        match *self {
            Number(value) => value,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_u8(&self) -> u8 {
        match *self {
            Number(value) => value as u8,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_u16(&self) -> u16 {
        match *self {
            Number(value) => value as u16,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_u32(&self) -> u32 {
        match *self {
            Number(value) => value as u32,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_u64(&self) -> u64 {
        match *self {
            Number(value) => value as u64,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_u128(&self) -> u128 {
        match *self {
            Number(value) => value as u128,
            _ => panic!("Number型ではありません"),
        }
    }

    pub fn as_usize(&self) -> usize {
        match *self {
            Number(value) => value as usize,
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
