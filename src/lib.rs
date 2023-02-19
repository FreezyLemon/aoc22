use std::{error::Error, fmt, path::PathBuf, str::FromStr};

pub fn get_input() -> Result<String, Box<dyn std::error::Error>> {
    // skip first arg (binary name)
    let mut args = std::env::args();
    let bin_name = args.next().unwrap();
    let path = PathBuf::from_str(&bin_name)?;

    let input_path = if let Some(arg) = args.next() {
        arg
    } else {
        // convenience
        format!("inputs/{}.txt", path.file_name().unwrap().to_string_lossy())
    };

    Ok(std::fs::read_to_string(input_path)?)
}

#[derive(Debug)]
struct ArgsError;

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not enough arguments provided")
    }
}

impl Error for ArgsError {}
