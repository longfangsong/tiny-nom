use crate::result::IResult;
use crate::many0::many0;
use crate::alt::alt;
use crate::tag::tag;
use crate::recognize::recognize;

pub fn multispace0<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    recognize(many0(alt((
        tag(" "),
        tag("\t"),
        tag("\r"),
        tag("\n")
    ))))(input)
}

#[test]
fn test_multispace0() {
    let test_case = multispace0("    \n\tHello?");
    assert_eq!(test_case, Ok(("Hello?", "    \n\t")));
    let test_case = multispace0("Hello?");
    assert_eq!(test_case, Ok(("Hello?", "")));
}