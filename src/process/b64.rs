use crate::{Base64DecodeOpts, Base64EncodeOpts, Base64Format};
use anyhow::Context;
use base64::prelude::*;
use std::{
    fs,
    io::{self, IsTerminal, Read, Write},
};

pub fn process_encode(opts: &Base64EncodeOpts) -> anyhow::Result<()> {
    let data = if opts.input == "-" {
        let mut buf = Vec::new();
        if io::stdin().is_terminal() {
            // 模式一：交互式终端
            eprintln!("请输入要编码的内容，按 Enter 结束：");
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            // 移除末尾的换行符
            buf.extend_from_slice(line.trim_end().as_bytes());
        } else {
            // 模式二：管道或重定向
            io::stdin().read_to_end(&mut buf)?;
        }
        buf
    } else {
        fs::read(&opts.input).with_context(|| format!("无法读取文件 '{}'", opts.input))?
    };

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
    let data = if opts.input == "-" {
        let mut buf = String::new();
        if io::stdin().is_terminal() {
            // 模式一：交互式终端
            eprintln!("请输入要解码的Base64内容，按 Enter 结束：");
            io::stdin().read_line(&mut buf)?;
        } else {
            // 模式二：管道或重定向
            io::stdin().read_to_string(&mut buf)?;
        }
        buf
    } else {
        fs::read_to_string(&opts.input).with_context(|| format!("无法读取文件 '{}'", opts.input))?
    };

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
