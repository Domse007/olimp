use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum CompilerError {
    MagicNotPresent,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::MagicNotPresent => write!(f, "Magic was not set."),
        }
    }
}

impl std::error::Error for CompilerError {}
