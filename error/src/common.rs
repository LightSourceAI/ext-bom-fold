use crate::error::Error;
use core::num::{ParseIntError, TryFromIntError};
use std::time::SystemTimeError;
use time::OutOfRangeError;

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::out_of_range(format!("Value exceeds 2^32: {e:?}"))
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::out_of_range(format!("Could not parse into integer: {e:?}"))
    }
}

impl From<OutOfRangeError> for Error {
    fn from(_: OutOfRangeError) -> Self {
        Error::out_of_range("Negative chrono durations cannot be converted to std::time::Duration")
    }
}

impl From<SystemTimeError> for Error {
    fn from(_: SystemTimeError) -> Self {
        Error::invalid_argument("SystemTime represents a point before the reference time")
    }
}
