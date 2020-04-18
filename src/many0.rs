use crate::result::IResult;
use std::result::Result::Ok;

pub fn many0<I: Clone + Copy, O, F1>(parse: F1) -> impl Fn(I) -> IResult<I, Vec<O>>
    where F1: Fn(I) -> IResult<I, O> {
    move |input| {
        let mut result = Vec::new();
        let mut current = input;
        while let Ok((rest, res)) = parse(current) {
            current = rest;
            result.push(res);
        }
        Ok((current, result))
    }
}

#[test]
fn test_many0() {
    use crate::tag::tag;
    let (rest, result) = many0(tag("1"))("111222").unwrap();
    assert_eq!(rest, "222");
    assert_eq!(result.len(), 3);
    let (rest, result) = many0(tag("2"))("111222").unwrap();
    assert_eq!(rest, "111222");
    assert_eq!(result.len(), 0);
}