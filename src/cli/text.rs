use super::{verify_file, verify_path};
use crate::{
    get_input_bytes, get_reader, process_text_key_generate, process_text_sign, process_text_verify,
    CmdExecutor,
};
use anyhow::{ensure, Context};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt, path::PathBuf, str::FromStr};
use tokio::fs;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign a text file")]
    Sign(TextSignOpts),
    #[command(about = "Verify a text file")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    // default_value = "-" 表示从stdin读取
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<TextSignFormat> for &str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_input_bytes(&self.key)?;
        let signature = process_text_sign(&mut *reader, &key, self.format)?;
        let encoded = URL_SAFE_NO_PAD.encode(signature);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_input_bytes(&self.key)?;
        let sig = URL_SAFE_NO_PAD.decode(self.sig.trim())?;
        let verified = process_text_verify(&mut *reader, &key, &sig, self.format)?;
        println!("{}", verified);
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let keys = process_text_key_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let key = keys
                    .get("blake3.txt")
                    .context("Blake3 generator did not return key material")?;
                fs::write(self.output.join("blake3.txt"), key).await?;
                eprintln!("Blake3 key generated successfully");
            }
            TextSignFormat::Ed25519 => {
                ensure!(
                    keys.contains_key("ed25519.sk") && keys.contains_key("ed25519.pk"),
                    "Ed25519 generator did not return key pair"
                );
                fs::write(
                    self.output.join("ed25519.sk"),
                    keys.get("ed25519.sk").unwrap(),
                )
                .await?;
                fs::write(
                    self.output.join("ed25519.pk"),
                    keys.get("ed25519.pk").unwrap(),
                )
                .await?;
                eprintln!("Ed25519 key pair generated successfully");
            }
        }
        Ok(())
    }
}
