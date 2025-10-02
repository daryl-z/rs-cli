use crate::CmdExecutor;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    #[command(about = "Sign a JWT token")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a JWT token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long, help = "Subject (sub claim)")]
    pub sub: String,
    #[arg(long, help = "Audience (aud claim)")]
    pub aud: String,
    #[arg(
        long,
        help = "Expiration time (e.g., 14d, 1h, 30m)",
        default_value = "14d"
    )]
    pub exp: String,
    #[arg(
        long,
        help = "HS256 shared secret. Falls back to JWT_SECRET env var when omitted"
    )]
    pub secret: Option<String>,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long, help = "JWT token to verify")]
    pub token: String,
    #[arg(long, help = "Expected audience (aud claim)")]
    pub aud: String,
    #[arg(
        long,
        help = "HS256 shared secret. Falls back to JWT_SECRET env var when omitted"
    )]
    pub secret: Option<String>,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = crate::process_jwt_sign(&self)?;
        println!("{}", token);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let valid = crate::process_jwt_verify(&self)?;
        println!("{}", valid);
        Ok(())
    }
}
