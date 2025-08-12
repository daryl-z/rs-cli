use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 12)]
    pub length: usize,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set, value_parser = clap::value_parser!(bool))]
    pub uppercase: bool,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set, value_parser = clap::value_parser!(bool))]
    pub lowercase: bool,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set, value_parser = clap::value_parser!(bool))]
    pub numbers: bool,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set, value_parser = clap::value_parser!(bool))]
    pub symbols: bool,
}
