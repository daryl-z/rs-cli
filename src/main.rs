use clap::Parser;

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
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_csv_opts() {
        let opts = CsvOpts::parse_from([
            "csv",
            "--input",
            "input.csv",
            "--output",
            "output.json",
            "--delimiter",
            ";",
            "--header",
        ]);
        assert_eq!(opts.input, "input.csv");
        assert_eq!(opts.output, "output.json");
        assert_eq!(opts.delimiter, ';');
        assert!(opts.header);
    }
}
