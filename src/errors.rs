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
    UnknownError(&'a str)
}