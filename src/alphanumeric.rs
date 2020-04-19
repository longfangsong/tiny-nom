use crate::alpha::alpha;
use crate::alt::alt;
use crate::digit::digit;
use crate::many0::many0;
use crate::recognize::recognize;
use crate::result::IResult;
use crate::tuple::tuple;

pub fn alphanumeric(input: &str) -> IResult<&str, char> {
    alt((digit, alpha))(input)
}

#[test]
fn test_alphanumeric() {
    let test_case = alphanumeric("abc");
    assert_eq!(test_case, Ok(("bc", 'a')));
    let test_case = alphanumeric("123");
    assert_eq!(test_case, Ok(("23", '1')));
    let test_case = alphanumeric("+123");
    assert!(test_case.is_err());
}

pub fn alphanumeric0(input: &str) -> IResult<&str, &str> {
    recognize(many0(alphanumeric))(input)
}

#[test]
fn test_alphanumeric0() {
    let test_case = alphanumeric0("abc");
    assert_eq!(test_case, Ok(("", "abc")));
    let test_case = alphanumeric0("123abc+");
    assert_eq!(test_case, Ok(("+", "123abc")));
    let test_case = alphanumeric0("+123");
    assert_eq!(test_case, Ok(("+123", "")));
}

pub fn alphanumeric1(input: &str) -> IResult<&str, &str> {
    recognize(tuple((recognize(alphanumeric), alphanumeric0)))(input)
}

#[test]
fn test_alphanumeric1() {
    let test_case = alphanumeric1("abc");
    assert_eq!(test_case, Ok(("", "abc")));
    let test_case = alphanumeric1("123abc+");
    assert_eq!(test_case, Ok(("+", "123abc")));
    let test_case = alphanumeric1("+123");
    assert!(test_case.is_err());
}