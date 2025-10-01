use crate::CmdExecutor;
use clap::Parser;
use zxcvbn::zxcvbn;

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

impl GenPassOpts {
    pub fn get_default_opts() -> Self {
        Self {
            length: 12,
            uppercase: true,
            lowercase: true,
            numbers: true,
            symbols: true,
        }
    }
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = crate::process_genpass(&self)?;
        println!("{}", password);
        let score = zxcvbn(&password, &[]).score();
        eprintln!("密码强度评分：{}/5", score);
        Ok(())
    }
}
