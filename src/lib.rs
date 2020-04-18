pub mod result;
pub mod tag;
pub mod alt;
pub mod map;
pub mod many0;
pub mod tuple;
pub mod recognize;
pub mod multispace0;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
