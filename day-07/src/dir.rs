#[derive(Debug, PartialEq, Eq)]
pub struct Dir<'a>(pub &'a str);

// either or
#[derive(Debug, PartialEq, Eq)]
pub enum Resources<'a> {
    File(&'a str, u32),
    Dir(&'a str),
}
