use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();  // program name is not necessary

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("missing query argument"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("missing file path argument"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
