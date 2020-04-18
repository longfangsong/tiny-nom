use crate::result::IResult;
use crate::tuple::list::TupleList;

mod list;

pub fn tuple<I, O, T: TupleList<I, O>>(parsers: T) -> impl Fn(I) -> IResult<I, O> {
    move |input| {
        parsers.parse(input)
    }
}

#[test]
fn tuple_test() {
    use crate::tag::tag;

    let test_case = tuple((tag("a"), tag("b")))("ab");
    assert_eq!(test_case, Ok(("", ("a", "b"))));
    let test_case = tuple((tag("a"), tag("b")))("abc");
    assert_eq!(test_case, Ok(("c", ("a", "b"))));
    let test_case = tuple((tag("c"), tag("d")))("abc");
    assert!(test_case.is_err());
}