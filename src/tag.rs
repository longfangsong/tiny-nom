use crate::result::{IResult, ErrorKind};

pub fn tag<'a>(tag: &'static str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        if input.len() >= tag.len() && input.starts_with(tag) {
            Ok((&input[tag.len()..], &input[..tag.len()]))
        } else {
            Err((input, ErrorKind::Tag))
        }
    }
}

#[test]
fn test_tag() {
    let test_case = tag("abc")("abcd");
    assert_eq!(test_case, Ok(("d", "abc")));
    let test_case = tag("abc")("dcba");
    assert!(test_case.is_err());
}