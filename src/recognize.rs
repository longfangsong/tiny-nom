use crate::result::{IResult, ErrorKind};

pub fn recognize<'a, O, F1>(parse: F1) -> impl Fn(&'a str) -> IResult<&'a str, &'a str>
    where F1: Fn(&'a str) -> IResult<&'a str, O> {
    move |input| {
        let parse_result = parse(input);
        if let Ok((after, _)) = parse_result {
            Ok((after, &input[0..(input.len() - after.len())]))
        } else {
            Err((input, ErrorKind::Recognize))
        }
    }
}