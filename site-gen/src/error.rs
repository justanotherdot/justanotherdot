use std::fmt::{Display, Formatter, Result};

pub enum Error {
    DeployDirectoryCreateError,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        use Error::*;
        match self {
            DeployDirectoryCreateError => write!(fmt, "failed creating deploy directories"),
        }
    }
}