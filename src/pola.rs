use polars::prelude::*;
use std::path::PathBuf;
use encoding_rs::WINDOWS_1252;





pub fn convert_to_df(file_path: Option<PathBuf>) -> PolarsResult<()> {
    let path = file_path.ok_or_else(|| PolarsError::NoData("no file path provided".into()))?;

    let bytes: Vec<u8> = std::fs::read(&path)?;
    let (decoded, _encoding_used, had_errors) = WINDOWS_1252.decode(&bytes);

    if had_errors {
        println!("Warning: some bytes couldn't be decoded cleanly");
    }

    let utf8_string = decoded.into_owned();
    std::fs::write("data_utf8.csv", &utf8_string)?;

    let df: DataFrame = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("data_utf8.csv".into()))?
        .finish()?;

    println!("{}", df);
    Ok(())
}
