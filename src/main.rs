use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

#[derive(Debug, Parser)]
#[clap(
    name = "rcli",
    version ,
    author,
    about ,
    long_about = None
)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file(file: &str) -> Result<String, String> {
    if Path::new(file).exists() {
        Ok(file.to_string())
    } else {
        Err(format!("文件 '{}' 不存在", file))
    }
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;

            // let records = reader.deserialize().map(|record| record.unwrap());
            // let records = reader.deserialize::<Player>().collect()?;
            let records = reader
                .deserialize::<Player>()
                .collect::<Result<Vec<Player>, _>>()?;
            println!("CSV Options: {:?}", records);
        }
    }
    Ok(())
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
