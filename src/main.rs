use clap::Parser;
use rs_cli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_generate,
    process_text_sign, process_text_verify, Base64SubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};
use std::fs;
use zxcvbn::zxcvbn;

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
            let pwd = process_genpass(&opts)?;
            println!("{}", pwd);
            let score = zxcvbn(&pwd, &[]).score();
            eprintln!("密码强度评分：{}/5", score);
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
                process_text_sign(&opts)?
            }
            TextSubCommand::Verify(opts) => {
                eprintln!("Opts: {:?}", opts);
                process_text_verify(&opts)?
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        eprintln!("Ed25519 key generated successfully");
                        // let name = &opts.output;
                        // fs::write(name.join("ed25519.sk"), &key[0])?;
                        // fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
