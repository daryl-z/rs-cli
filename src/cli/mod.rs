mod base64;
mod csv;
mod genpass;
mod text;

use std::path::Path;

// 重新导出子模块的公共类型，提供统一接口
pub use base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand};
use clap::Parser;
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;
pub use text::TextSubCommand;

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
    #[command(subcommand)]
    Text(TextSubCommand),
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
