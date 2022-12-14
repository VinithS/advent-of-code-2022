#[derive(Debug, PartialEq, Eq)]
pub struct Dir<'a>(pub &'a str);

pub const DELIMITER: &str = "/";
pub const ROOT_DIR: &str = DELIMITER;

// either or
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Resources<'a> {
    File(&'a str, u32),
    Dir(&'a str),
}

pub fn generate_path(dirs: &Vec<&str>) -> String {
    if dirs.is_empty() || dirs.len() == 1 {
        return "/".to_string();
    }
    let full_path = dirs.join(DELIMITER);

    if !full_path.starts_with('/') {
        return ["/", &full_path].join("");
    }
    full_path
}

impl Resources<'_> {
    pub fn is_file(&self) -> bool {
        matches!(self, Resources::File(..))
    }
}

#[cfg(test)]
mod tests {
    use crate::dir::Resources;

    use super::generate_path;

    #[test]
    fn test_is_file() {
        assert_eq!(true, Resources::File("a", 123).is_file());
        assert_eq!(false, Resources::Dir("a").is_file());
    }

    #[test]
    fn test_path_generation() {
        assert_eq!("/a/b/c", generate_path(&vec!["", "a", "b", "c"]));
        assert_eq!("/a/b/c", generate_path(&vec!["a", "b", "c"]));
        assert_eq!("/", generate_path(&vec![""]));
    }
}
