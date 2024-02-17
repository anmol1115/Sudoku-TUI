#[derive(Debug)]
pub enum Errors<'a> {
    FileNotFoundError(&'a str),
    CsvUnpackingError(&'a str),
    ConversionError(&'a str),
    FileCreationError(&'a str),
    FileWriteError(&'a str),
    DirectoryCreationError(&'a str),
    UnknownError(&'a str)
}