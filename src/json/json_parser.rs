use crate::json::lexical_analysis::decode_result::DecodeResult;
use crate::json::lexical_analysis::lexical_analyzer::LexicalAnalysis;

pub trait JsonDecoder<T> {
    fn parser(result: &DecodeResult) -> T;

    fn decode_from(json: &str) -> T {
        let result = LexicalAnalysis::extract(json).unwrap().1;
        Self::parser(&result)
    }
}

impl JsonDecoder<i8> for i8 {
    fn parser(result: &DecodeResult) -> i8 {
        result.as_i8()
    }
}

impl JsonDecoder<i16> for i16 {
    fn parser(result: &DecodeResult) -> i16 {
        result.as_i16()
    }
}

impl JsonDecoder<i32> for i32 {
    fn parser(result: &DecodeResult) -> i32 {
        result.as_i32()
    }
}

impl JsonDecoder<i64> for i64 {
    fn parser(result: &DecodeResult) -> i64 {
        result.as_i64()
    }
}

impl JsonDecoder<i128> for i128 {
    fn parser(result: &DecodeResult) -> i128 {
        result.as_i128()
    }
}

impl JsonDecoder<isize> for isize {
    fn parser(result: &DecodeResult) -> isize {
        result.as_isize()
    }
}

impl JsonDecoder<u8> for u8 {
    fn parser(result: &DecodeResult) -> u8 {
        result.as_u8()
    }
}

impl JsonDecoder<u16> for u16 {
    fn parser(result: &DecodeResult) -> u16 {
        result.as_u16()
    }
}

impl JsonDecoder<u32> for u32 {
    fn parser(result: &DecodeResult) -> u32 {
        result.as_u32()
    }
}

impl JsonDecoder<u64> for u64 {
    fn parser(result: &DecodeResult) -> u64 {
        result.as_u64()
    }
}

impl JsonDecoder<u128> for u128 {
    fn parser(result: &DecodeResult) -> u128 {
        result.as_u128()
    }
}

impl JsonDecoder<usize> for usize {
    fn parser(result: &DecodeResult) -> usize {
        result.as_usize()
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
