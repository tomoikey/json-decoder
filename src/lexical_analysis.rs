use nom::branch::permutation;
use nom::character::complete::{alphanumeric1, char, digit1, multispace0};
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug)]
pub struct LexicalAnalysis {
    value: String,
}

impl<'a> LexicalAnalysis {
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }

    fn try_to_extract_string(input: &str) -> IResult<&str, Option<DecodeResult>> {
        let (remains, value) = opt(delimited(
            multispace0,
            delimited(char('\"'), alphanumeric1, char('\"')),
            multispace0,
        ))(input)?;
        Ok((remains, value.map(|n| DecodeResult::Str(n.to_string()))))
    }

    fn try_to_extract_digit(input: &str) -> IResult<&str, Option<DecodeResult>> {
        let (remains, value) = opt(delimited(multispace0, digit1, multispace0))(input)?;
        Ok((
            remains,
            value.map(|n| DecodeResult::Number(n.parse::<usize>().unwrap())),
        ))
    }

    fn try_to_extract_array(input: &str) -> IResult<&str, Option<DecodeResult>> {
        let (remains, value) = opt(delimited(
            delimited(multispace0, char('['), multispace0),
            separated_list0(
                permutation((multispace0, char(','), multispace0)),
                |input| {
                    let input: &str = input;
                    let (remains, value) = digit1(input)?;
                    Ok((remains, value))
                },
            ),
            delimited(multispace0, char(']'), multispace0),
        ))(input)?;
        Ok((
            remains,
            value.map(|n| {
                DecodeResult::Array(n.into_iter().map(|m| m.to_string()).collect::<Vec<_>>())
            }),
        ))
    }

    pub fn extract(&self) -> IResult<&str, Vec<(&str, DecodeResult)>> {
        let value = self.value.as_str();
        let (remains, _) = delimited(multispace0, char('{'), multispace0)(value)?;
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
                let (remains, value) = Self::try_to_extract_digit(remains)?;
                if let Some(value) = value {
                    Ok((remains, (key, value)))
                } else {
                    let (remains, value) = Self::try_to_extract_string(remains)?;
                    if let Some(value) = value {
                        Ok((remains, (key, value)))
                    } else {
                        let (remains, value) = Self::try_to_extract_array(remains)?;
                        if let Some(value) = value {
                            Ok((remains, (key, value)))
                        } else {
                            todo!()
                        }
                    }
                }
            },
        )(remains)?;
        let (remains, _) = delimited(multispace0, char('}'), multispace0)(remains)?;
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
