use std::io;

#[derive(Debug)]
pub enum Errors<'a> {
    FileNotFoundError(&'a str),
    CsvUnpackingError(&'a str),
    ConversionError(&'a str),
    FileCreationError(&'a str),
    FileWriteError(&'a str),
    FileReadError(&'a str),
    DirectoryCreationError(&'a str),
    DirectoryNotFoundError(&'a str),
    DirectoryListError(&'a str),
    UnknownError(&'a str)
}

impl<'a> From<Errors<'a>> for io::Error {
    fn from(error: Errors) -> Self {
        match error {
            Errors::FileNotFoundError(message) | Errors::CsvUnpackingError(message) | Errors::ConversionError(message) | Errors::FileCreationError(message) | Errors::FileWriteError(message) | Errors::FileReadError(message) | Errors::DirectoryCreationError(message) | Errors::DirectoryNotFoundError(message) | Errors::DirectoryListError(message) | Errors::UnknownError(message) => io::Error::new(io::ErrorKind::Other, message)
        }
    }
}