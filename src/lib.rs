mod cli;
mod process;
mod utils;

pub use crate::cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand, CsvOpts, GenPassOpts,
    HttpServeOpts, HttpSubCommand, Opts, OutputFormat, SubCommand, TextKeyGenerateOpts,
    TextSignFormat, TextSignOpts, TextSubCommand, TextVerifyOpts,
};
use enum_dispatch::enum_dispatch;

pub use crate::process::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify,
};
pub use crate::utils::{get_input_bytes, get_input_string, get_reader};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
