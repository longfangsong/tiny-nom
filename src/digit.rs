use crate::many0::many0;
use crate::recognize::recognize;
use crate::result::{ErrorKind, IResult};
use crate::tuple::tuple;

pub fn digit(input: &str) -> IResult<&str, char> {
    if let Some(first) = input.chars().next() {
        if first.is_digit(10) {
            Ok((&input[1..], first))
        } else {
            Err((input, ErrorKind::Digit))
        }
    } else {
        Err((input, ErrorKind::Digit))
    }
}

#[test]
fn test_digit() {
    let test_case = digit("123");
    assert_eq!(test_case, Ok(("23", '1')));
    let test_case = digit("abc");
    assert!(test_case.is_err());
}

pub fn digit0(input: &str) -> IResult<&str, &str> {
    recognize(many0(digit))(input)
}

#[test]
fn test_digit0() {
    let test_case = digit0("123");
    assert_eq!(test_case, Ok(("", "123")));
    let test_case = digit0("abc");
    assert_eq!(test_case, Ok(("abc", "")));
}

pub fn digit1(input: &str) -> IResult<&str, &str> {
    recognize(tuple((recognize(digit), digit0)))(input)
}

#[test]
fn test_digit1() {
    let test_case = digit1("123");
    assert_eq!(test_case, Ok(("", "123")));
    let test_case = digit1("abc");
    assert!(test_case.is_err());
}