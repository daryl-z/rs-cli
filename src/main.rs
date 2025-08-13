use clap::Parser;
use rs_cli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(&opts)?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts)?;
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                eprintln!("Opts: {:?}", opts);
                // process_sign(&opts)?;
            }
            TextSubCommand::Verify(opts) => {
                eprintln!("Opts: {:?}", opts);
                // process_verify(&opts)?;
            }
        },
    }
    Ok(())
}
