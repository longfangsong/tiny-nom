use crate::many0::many0;
use crate::recognize::recognize;
use crate::result::{ErrorKind, IResult};
use crate::tuple::tuple;

pub fn alpha(input: &str) -> IResult<&str, char> {
    if let Some(first) = input.chars().next() {
        if first.is_ascii_alphabetic() {
            Ok((&input[1..], first))
        } else {
            Err((input, ErrorKind::Alpha))
        }
    } else {
        Err((input, ErrorKind::Alpha))
    }
}

#[test]
fn test_alpha() {
    let test_case = alpha("abc");
    assert_eq!(test_case, Ok(("bc", 'a')));
    let test_case = alpha("123");
    assert!(test_case.is_err());
}

pub fn alpha0(input: &str) -> IResult<&str, &str> {
    recognize(many0(alpha))(input)
}

#[test]
fn test_alpha0() {
    let test_case = alpha0("abc");
    assert_eq!(test_case, Ok(("", "abc")));
    let test_case = alpha0("123");
    assert_eq!(test_case, Ok(("123", "")));
}

pub fn alpha1(input: &str) -> IResult<&str, &str> {
    recognize(tuple((recognize(alpha), alpha0)))(input)
}

#[test]
fn test_alpha1() {
    let test_case = alpha1("abc");
    assert_eq!(test_case, Ok(("", "abc")));
    let test_case = alpha1("123");
    assert!(test_case.is_err());
}