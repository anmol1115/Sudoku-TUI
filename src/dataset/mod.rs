use csv;
use std::{
    fs::{self, File, OpenOptions},
    io::Write
};

use crate::errors::Errors;

fn write_to_file(file_idx: i32, problem: &str, solution: &str) -> Result<(), Errors<'static>> {
    let mut file =  match OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("./problems/{file_idx}.csv")) {
            Ok(file_handle) => file_handle,
            Err(_) => return Err(Errors::FileCreationError("Unable to create a new file"))
        };

        if let Err(_) = file.write_all(format!("{problem},{solution}\n").as_bytes()){
            return Err(Errors::FileWriteError("Unable to write to file"))
        };
        Ok(())
}

fn create_dataset(path: &str) -> Result<(), Errors> {
    if let Err(_) = fs::create_dir("./problems") {
        return Err(Errors::DirectoryCreationError("Error Creating problems directory"))
    }

    let file = match File::open(path) {
        Ok(file_handler) => file_handler,
        Err(_) => return Err(Errors::FileNotFoundError("Error opening dataset file, please ensure the dataset is available in the project root"))
    };

    let mut csv_reader = csv::ReaderBuilder::new()
        .from_reader(file);

    for res in csv_reader.records() {
        let record = match res {
            Ok(rec) => rec,
            Err(_) => return Err(Errors::CsvUnpackingError("Error unpacking values from dataset file, please ensure the file format is correct (i.e. csv)"))
        };
        let record: Vec<String> = record.iter().map(|s| s.to_string()).collect();

        if let [_, prob, sol, _, diff] = &record[..] {
            let diff: f32 = match diff.parse() {
                Ok(diff) => diff,
                Err(_) => return Err(Errors::ConversionError("Error converting string to float"))
            };

            let diff = diff.trunc() as i32;
            write_to_file(diff, prob, sol)?;
        } else {
            return Err(Errors::UnknownError("HOW!!"))
        }
    }

    Ok(())
}

pub fn initialize() -> Result<(), Errors<'static>> {
    if let Ok(metadata) = fs::metadata("./problems") {
        if !metadata.is_dir() {
            create_dataset("./sample-sudoku.csv")?;
        }
    } else {
        create_dataset("./sample-sudoku.csv")?;
    }

    Ok(())
}