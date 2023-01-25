use nom::branch::permutation;
use nom::character::complete::{alphanumeric1, char, digit1, multispace0};
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug)]
pub struct LexicalAnalysis {}

impl<'a> LexicalAnalysis {
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

    pub fn extract(input: &str) -> IResult<&str, DecodeResult> {
        let (remains, _) = delimited(multispace0, char('{'), multispace0)(input)?;
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
                            let (remains, value) = Self::extract(remains)?;
                            Ok((remains, (key, value)))
                        }
                    }
                }
            },
        )(remains)?;
        let (remains, _) = delimited(multispace0, char('}'), multispace0)(remains)?;
        let value = value
            .into_iter()
            .map(|n| (n.0.to_string(), Box::new(n.1)))
            .collect();
        Ok((remains, DecodeResult::Json(value)))
    }
}

#[derive(Debug, PartialEq)]
pub enum DecodeResult {
    Str(String),
    Number(usize),
    Array(Vec<String>),
    Json(Vec<(String, Box<DecodeResult>)>),
}

#[test]
fn should_extract() {
    use DecodeResult::*;
    let json = "{ \"age\": 1, \"name\": \"Tom\", \"array\": [1, 2, 4, 3] }";
    let la = LexicalAnalysis::extract(json);
    let expected = vec![
        (String::from("age"), Number(1)),
        (String::from("name"), Str(String::from("Tom"))),
        (
            String::from("array"),
            Array(vec![
                String::from("1"),
                String::from("2"),
                String::from("4"),
                String::from("3"),
            ]),
        ),
    ]
    .into_iter()
    .map(|n| (n.0, Box::new(n.1)))
    .collect();
    assert_eq!(la, Ok(("", Json(expected))))
}
