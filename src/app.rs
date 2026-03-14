use crate::{config, search};

use std::{error::Error, fs};

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let matches = if config.ignore_case {
        search::search_case_insens(&config.query, &content)
    } else {
        search::search(&config.query, &content)
    };

    for line in matches {
        println!("{line}")
    }

    Ok(())
}
