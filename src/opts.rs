use clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(file: &str) -> Result<String, String> {
    if Path::new(file).exists() {
        Ok(file.to_string())
    } else {
        Err(format!("文件 '{}' 不存在", file))
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
        assert_eq!(opts.output, "output.json");
        assert_eq!(opts.delimiter, ';');
        assert!(opts.header);
    }
}
