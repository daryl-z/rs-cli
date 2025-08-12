use clap::Parser;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[clap(
    name = "rcli",
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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 12)]
    pub length: usize,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub numbers: bool,
    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

fn verify_input_file(file: &str) -> Result<String, String> {
    if Path::new(file).exists() {
        Ok(file.to_string())
    } else {
        Err(format!("文件 '{}' 不存在", file))
    }
}

fn parse_format(fmt: &str) -> Result<OutputFormat, anyhow::Error> {
    fmt.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => anyhow::bail!("无效的格式： {}", value),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    #[test]
    fn test_csv_opts() {
        let tmp = NamedTempFile::new().expect("failed to create temp file");
        let tmp_path = tmp.path().to_str().expect("temp path utf8").to_string();

        let opts = CsvOpts::parse_from([
            "csv",
            "--input",
            tmp_path.as_str(),
            "--output",
            "output.json",
            "--delimiter",
            ";",
            "--header",
        ]);
        assert_eq!(opts.input, tmp.path().to_str().unwrap());
        // assert_eq!(opts.output, "output.json");
        assert_eq!(opts.delimiter, ';');
        assert!(opts.header);
    }
}
