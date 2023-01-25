use nom::branch::permutation;
use nom::character::complete::{alphanumeric0, alphanumeric1, anychar, char, digit1, multispace0};
use nom::combinator::opt;
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

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

    pub fn extract(&self) -> IResult<&str, Vec<(&str, &str)>> {
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
                if value.is_none() {
                    let (remains, value) = delimited(
                        multispace0,
                        delimited(char('\"'), alphanumeric1, char('\"')),
                        multispace0,
                    )(remains)?;
                    Ok((remains, (key, value)))
                } else {
                    Ok((remains, (key, value.unwrap())))
                }
            },
        )(remains)?;
        let (remains, _) = char('}')(remains)?;
        Ok((remains, value))
    }
}

#[test]
fn should_extract() {
    let json = "{ \"age\": 1, \"name\": \"Tom\", \"array\": [1, 2, 4, 3] }";
    let la = LexicalAnalysis::new("{123}");
    assert_eq!(Ok(todo!()), la.extract())
}
