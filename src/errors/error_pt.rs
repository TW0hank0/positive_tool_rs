use std::fmt;

#[derive(Debug)]
pub enum PTErrors {
    FindProjectPathError(String),
}

impl fmt::Display for PTErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PTErrors::FindProjectPathError(msg) => {
                return write!(f, "find_projct_path錯誤： {}", msg);
            }
        }
    }
}

impl std::error::Error for PTErrors {}
