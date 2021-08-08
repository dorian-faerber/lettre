//! Error and result type for emails

use self::Error::*;
use std::error::Error as StdError;
use std::io;
use std::fmt::{self, Display, Formatter};

/// An enum of all error kinds.
#[derive(Debug)]
pub enum Error {
    /// Missing sender
    MissingFrom,
    /// Missing recipient
    MissingTo,
    /// Unparseable filename for attachment
    CannotParseFilename,
    /// IO error
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&self.to_string())
    }
}

impl StdError for Error {
    /*fn description(&self) -> &str {
        match *self {
            MissingFrom => "the sender is missing",
            MissingTo => "the recipient is missing",
            CannotParseFilename => "the attachment filename could not be parsed",
            Io(ref err) => &err.to_string(),
        }
    }*/
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Io(err)
    }
}

