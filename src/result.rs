#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Tag,
    Alt,
    Recognize,
    Tuple
}

pub type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), E>;