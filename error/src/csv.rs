use crate::Error;

impl From<csv::Error> for Error {
    fn from(csv_error: csv::Error) -> Error {
        match csv_error.into_kind() {
            csv::ErrorKind::UnequalLengths { .. } => {
                Error::invalid_argument("CSV contains records with unequal number of fields")
            }
            csv::ErrorKind::Deserialize { .. } => {
                Error::invalid_argument("Could not deserialize CSV")
            }
            csv::ErrorKind::Serialize(_) => Error::invalid_argument("Could not serialize CSV"),
            csv::ErrorKind::Io(_) => Error::internal("I/O error occurred while reading CSV data."),
            csv::ErrorKind::Seek => {
                Error::internal("Reader asked to seek before first record is parsed")
            }
            csv::ErrorKind::Utf8 { .. } => {
                Error::internal("UTF-8 decoding error while trying to read CSV data")
            }
            csv::ErrorKind::__Nonexhaustive => {
                Error::unknown("Unknown error occurred with csv library")
            }
        }
    }
}

impl<W> From<csv::IntoInnerError<W>> for Error {
    fn from(csv_error: csv::IntoInnerError<W>) -> Self {
        Error::internal(csv_error.to_string())
    }
}
