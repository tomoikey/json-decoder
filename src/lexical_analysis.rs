use nom::branch::permutation;
use nom::character::complete::{alphanumeric0, alphanumeric1, anychar, char, digit1, multispace0};
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug)]
pub struct LexicalAnalysis {
    value: String,
}

impl<'a> LexicalAnalysis {
    const UPPER_ALPHA: &'a str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER_ALPHA: &'a str = "abcdefghijklmnopqrstuvwxyz";
    const DOUBLE_QUOTATION: &'a str = "\"";
    const COMMA: &'a str = ",";
    const ARRAY_EXPR: &'a str = "[]";

    pub fn new<T: Into<String>>(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn extract(&self) -> IResult<&str, Vec<(&str, DecodeResult)>> {
        let value = self.value.as_str();
        let (remains, _) = char('{')(value)?;
        let (remains, value) = separated_list0(
            permutation((multispace0, char(','), multispace0)),
            |input| {
                let input: &str = input;
                let (remains, key) = delimited(
                    multispace0,
                    delimited(char('\"'), alphanumeric1, char('\"')),
                    multispace0,
                )(input)?;
                let (remains, _) = char(':')(remains)?;
                let (remains, value) = opt(delimited(multispace0, digit1, multispace0))(remains)?;
                if let Some(value) = value {
                    Ok((
                        remains,
                        (key, DecodeResult::Number(value.parse::<usize>().unwrap())),
                    ))
                } else {
                    let (remains, value) = opt(delimited(
                        multispace0,
                        delimited(char('\"'), alphanumeric1, char('\"')),
                        multispace0,
                    ))(remains)?;
                    if let Some(value) = value {
                        Ok((remains, (key, DecodeResult::Str(value.to_string()))))
                    } else {
                        let (remains, _) = char('[')(remains)?;
                        let (remains, value) = separated_list0(
                            permutation((multispace0, char(','), multispace0)),
                            |input| {
                                let input: &str = input;
                                let (remains, value) = digit1(input)?;
                                Ok((remains, value))
                            },
                        )(remains)?;
                        let (remains, _) = char(']')(remains)?;
                        Ok((
                            remains,
                            (
                                key,
                                DecodeResult::Array(
                                    value.into_iter().map(|n| n.to_string()).collect(),
                                ),
                            ),
                        ))
                    }
                }
            },
        )(remains)?;
        let (remains, _) = char('}')(remains)?;
        Ok((remains, value))
    }
}

#[derive(Debug)]
pub enum DecodeResult {
    Str(String),
    Number(usize),
    Array(Vec<String>),
}

#[test]
fn should_extract() {
    // let json = "{ \"age\": 1, \"name\": \"Tom\", \"array\": [1, 2, 4, 3] }";
    // let la = LexicalAnalysis::new("{123}");
    // assert_eq!(Ok(todo!()), la.extract())
}
