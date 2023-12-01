use core::fmt;


#[derive(Debug)]
pub struct FplError {
    msg: String,
}

impl fmt::Display for FplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FplError: {}", self.msg)
    }
}

impl From<&str> for FplError {
    fn from(item: &str) -> Self {
        FplError {
            msg: item.to_string(),
        }
    }
}
