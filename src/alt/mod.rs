use crate::result::{IResult, ErrorKind};
use crate::alt::list::AltList;

mod list;

pub fn alt<I: Copy, O, T: AltList<I, O>>(parsers: T) -> impl Fn(I) -> IResult<I, O> {
    move |input| {
        let parser_count = parsers.len();
        for i in 0..parser_count {
            if let Ok(it) = parsers.apply_nth(i, input) { return Ok(it); }
        }
        Err((input, ErrorKind::Alt))
    }
}

#[test]
fn test_alt() {
    use crate::tag::tag;

    let test_case = alt((tag("a"), tag("b")))("ab");
    assert_eq!(test_case, Ok(("b", "a")));
    let test_case = alt((tag("a"), tag("b")))("ba");
    assert_eq!(test_case, Ok(("a", "b")));
    let test_case = alt((tag("a"), tag("b")))("cd");
    assert!(test_case.is_err());
}