use polars::prelude::*;
use std::path::PathBuf;
use encoding_rs::WINDOWS_1252;





pub fn convert_to_df(file_path: Option<PathBuf>) -> PolarsResult<()> {
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(file_path)?
        .finish()?;

    println!("{}", df);

    Ok(())
}


fn convert_to_df_with_encoding(file_path: Option<PathBuf>, encoding: &'static Encoding) -> PolarsResult<()> {
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(file_path)?
        .with_encoding(encoding)
        .finish()?;

    println!("{}", df);

    Ok(())
}