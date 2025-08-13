mod cli;
mod process;
mod utils;

pub use crate::cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand, CsvOpts, GenPassOpts, Opts,
    OutputFormat, SubCommand, TextSubCommand,
};

pub use crate::process::{process_csv, process_decode, process_encode, process_genpass};
pub use crate::utils::{get_input_bytes, get_input_string, get_reader};
