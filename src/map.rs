use crate::result::IResult;

pub fn map<I, O1, O, F1, F2>(parse: F1, transform: F2) -> impl Fn(I) -> IResult<I, O>
    where F1: Fn(I) -> IResult<I, O1>,
          F2: Fn(O1) -> O {
    move |input| {
        let parse_result = parse(input);
        match parse_result {
            Ok((rest, matched)) => Ok((rest, transform(matched))),
            Err(err) => Err(err)
        }
    }
}

#[test]
fn test_map() {
    use crate::tag::tag;
    use crate::alt::alt;
    use std::str::FromStr;
    
    let test_parser = map(alt((tag("1234"), tag("5678"))), |t| u32::from_str(t).unwrap());
    let test_case = test_parser("1234");
    assert_eq!(test_case, Ok(("", 1234u32)));
    let test_case = test_parser("12345678");
    assert_eq!(test_case, Ok(("5678", 1234u32)));
    let test_case = test_parser("56781234");
    assert_eq!(test_case, Ok(("1234", 5678u32)));
    let test_case = test_parser("abcd");
    assert!(test_case.is_err());
}
