mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::{
    base64::Base64DecodeOpts, base64::Base64EncodeOpts, base64::Base64Format,
    base64::Base64SubCommand, csv::CsvOpts, csv::OutputFormat, genpass::GenPassOpts,
};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "rst",
    version ,
    author,
    about ,
    long_about = None
)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("文件 '{}' 不存在", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(
            verify_input_file("nonexistent"),
            Err("文件 'nonexistent' 不存在".into())
        );
    }
}
