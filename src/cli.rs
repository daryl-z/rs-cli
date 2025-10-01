mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

// 重新导出子模块的公共类型，提供统一接口
pub use base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand};
use clap::Parser;
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;
pub use http::{HttpServeOpts, HttpSubCommand};
pub use text::{TextKeyGenerateOpts, TextSignFormat, TextSignOpts, TextSubCommand, TextVerifyOpts};

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
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

pub fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("文件 '{}' 不存在", filename))
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(
            verify_file("nonexistent"),
            Err("文件 'nonexistent' 不存在".into())
        );
    }
}
