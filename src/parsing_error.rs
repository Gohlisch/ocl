#[derive(Debug)]
pub struct ParsingError<'a> {
    pub msg: &'a str,
    pub from: usize,
    pub to: usize
}