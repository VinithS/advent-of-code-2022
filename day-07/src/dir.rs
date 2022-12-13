#[derive(Debug)]
pub struct Dir<'a>(pub &'a str);

#[derive(Debug)]
pub struct File<'a>(pub &'a str, pub u32);

// either or
#[derive(Debug)]
pub enum Resources<'a> {
    File(File<'a>),
    Dir(&'a str),
}
