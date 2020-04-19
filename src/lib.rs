pub mod result;
pub mod tag;
pub mod alt;
pub mod map;
pub mod many0;
pub mod tuple;
pub mod recognize;
pub mod multispace0;
pub mod alpha;
pub mod digit;
pub mod alphanumeric;

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::fmt::Formatter;

    use crate::alpha::alpha1;
    use crate::alphanumeric::alphanumeric0;
    use crate::alt::alt;
    use crate::digit::digit1;
    use crate::many0::many0;
    use crate::map::map;
    use crate::multispace0::multispace0;
    use crate::recognize::recognize;
    use crate::result::IResult;
    use crate::tag::tag;
    use crate::tuple::tuple;

    #[derive(Debug, Eq, PartialEq)]
    pub enum Operator {
        Plus,
        Minus,
        Times,
        Slash,
        Eql,
        Neq,
        Lss,
        Leq,
        Gtr,
        Geq,
        Becomes,
    }

    impl fmt::Display for Operator {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            write!(f, "{}", format!("{:?}", self).to_ascii_lowercase())
        }
    }

    pub fn operator(code: &str) -> IResult<&str, Operator> {
        alt((
            map(tag("+"), |_| Operator::Plus),
            map(tag("-"), |_| Operator::Minus),
            map(tag("*"), |_| Operator::Times),
            map(tag("/"), |_| Operator::Slash),
            map(tag("="), |_| Operator::Eql),
            map(tag("#"), |_| Operator::Neq),
            map(tag("<="), |_| Operator::Leq),
            map(tag("<"), |_| Operator::Lss),
            map(tag(">="), |_| Operator::Geq),
            map(tag(">"), |_| Operator::Gtr),
            map(tag(":="), |_| Operator::Becomes)
        ))(code)
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Token {
        Ident(String),
        Number(String),
        Operator(Operator),
    }

    fn ident(code: &str) -> IResult<&str, Token> {
        map(recognize(tuple((
            alpha1,
            alphanumeric0
        ))), |it: &str| Token::Ident(it.to_string()))(code)
    }

    fn number(code: &str) -> IResult<&str, Token> {
        map(recognize(digit1), |it: &str| Token::Number(it.to_string()))(code)
    }

    fn token(code: &str) -> IResult<&str, Token> {
        alt((
            map(operator, |it| Token::Operator(it)),
            ident,
            number
        ))(code)
    }

    fn token_stream(code: &str) -> IResult<&str, Vec<Token>> {
        many0(map(tuple((
            multispace0,
            token,
            multispace0
        )), |(_, result, _)| result))(code)
    }

    #[test]
    fn it_works() {
        let test_case = token_stream("a + 1").unwrap();
        assert_eq!(test_case.0, "");
        assert_eq!(test_case.1[0], Token::Ident("a".to_string()));
        assert_eq!(test_case.1[1], Token::Operator(Operator::Plus));
        assert_eq!(test_case.1[2], Token::Number("1".to_string()));
    }
}
