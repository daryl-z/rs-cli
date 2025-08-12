mod cli;
mod process;

pub use crate::process::b64::process_decode;
pub use crate::process::b64::process_encode;
pub use crate::process::csv_convert::process_csv;
pub use crate::process::gen_pass::process_genpass;
pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand, CsvOpts, Opts,
    OutputFormat, SubCommand,
};
