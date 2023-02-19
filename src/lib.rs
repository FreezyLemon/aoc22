use std::{error::Error, fmt};

pub fn get_input() -> Result<String, Box<dyn std::error::Error>> {
    // skip first arg (binary name)
    let mut args = std::env::args().skip(1);

    if let Some(arg) = args.next() {
        let content = std::fs::read_to_string(arg)?;
        Ok(content)
    } else {
        Err(Box::new(ArgsError))
    }
}

#[derive(Debug)]
struct ArgsError;

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not enough arguments provided")
    }
}

impl Error for ArgsError {}
