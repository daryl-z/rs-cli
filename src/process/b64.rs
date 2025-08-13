use crate::{get_input_bytes, get_input_string, Base64DecodeOpts, Base64EncodeOpts, Base64Format};
use anyhow::Context;
use base64::prelude::*;
use std::io::{self, Write};

pub fn process_encode(opts: &Base64EncodeOpts) -> anyhow::Result<()> {
    let data =
        get_input_bytes(&opts.input).with_context(|| format!("无法读取输入源 '{}'", opts.input))?;

    if data.is_empty() {
        eprintln!("警告：输入内容为空，没有数据需要编码。");
        return Ok(());
    }

    let encoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.encode(&data),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(&data),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(opts: &Base64DecodeOpts) -> anyhow::Result<()> {
    let data = get_input_string(&opts.input)
        .with_context(|| format!("无法读取输入源 '{}'", opts.input))?;

    let trimmed_data = data.trim();
    if trimmed_data.is_empty() {
        eprintln!("警告：输入内容为空，没有数据需要解码。");
        return Ok(());
    }

    let decoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.decode(trimmed_data),
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(trimmed_data),
    }
    .context("Base64 解码失败，请检查输入格式是否正确")?;

    // 尝试以 UTF-8 字符串输出，如果失败则输出原始字节
    match String::from_utf8(decoded.clone()) {
        Ok(text) => {
            println!("{}", text);
            io::stdout().flush()?;
        }
        Err(_) => {
            io::stdout().write_all(&decoded)?;
        }
    }

    Ok(())
}
