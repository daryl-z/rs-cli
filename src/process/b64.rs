use crate::{Base64DecodeOpts, Base64EncodeOpts, Base64Format};

use base64::prelude::*;
use std::{
    fs::File,
    io::{IsTerminal, Read},
};

pub fn process_encode(opts: &Base64EncodeOpts) -> anyhow::Result<()> {
    let data = if opts.input == "-" {
        if std::io::stdin().is_terminal() {
            eprintln!("请输入要编码的内容 (按回车键结束):");
        }
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => {
                eprintln!("警告：没有读取到任何内容");
                return Ok(());
            }
            Ok(_) => {
                // 移除末尾的换行符
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                line.into_bytes()
            }
            Err(e) => {
                eprintln!("从标准输入读取数据时出错：{}", e);
                eprintln!("请检查输入或尝试使用文件输入：-i filename");
                return Ok(());
            }
        }
    } else {
        let mut file = match File::open(&opts.input) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("无法打开文件 '{}'：{}", opts.input, e);
                eprintln!("请检查：");
                eprintln!("1. 文件是否存在");
                eprintln!("2. 是否有读取权限");
                eprintln!("3. 路径是否正确");
                return Ok(());
            }
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(size) => {
                if size == 0 {
                    eprintln!("警告：文件 '{}' 为空", opts.input);
                }
                if size > 10 * 1024 * 1024 {
                    // 10MB
                    eprintln!(
                        "提示：正在编码大文件 ({:.2} MB)，请稍候...",
                        size as f64 / 1024.0 / 1024.0
                    );
                }
                data
            }
            Err(e) => {
                eprintln!("读取文件 '{}' 时出错：{}", opts.input, e);
                return Ok(());
            }
        }
    };

    // 检查数据是否为空
    if data.is_empty() {
        eprintln!("没有数据需要编码");
        return Ok(());
    }

    let encoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.encode(data),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(data),
    };
    println!("Encoded: {}", encoded);
    Ok(())
}

pub fn process_decode(opts: &Base64DecodeOpts) -> anyhow::Result<()> {
    let input_str = if opts.input == "-" {
        if std::io::stdin().is_terminal() {
            eprintln!("请输入要解码的Base64内容 (按回车键结束):");
        }
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => {
                eprintln!("警告：没有读取到任何内容");
                return Ok(());
            }
            Ok(_) => {
                let trimmed = line.trim();
                trimmed.to_string()
            }
            Err(e) => {
                eprintln!("从标准输入读取数据时出错：{}", e);
                return Ok(());
            }
        }
    } else {
        let mut file = match File::open(&opts.input) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("无法打开文件 '{}'：{}", opts.input, e);
                return Ok(());
            }
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(size) => {
                if size == 0 {
                    eprintln!("错误：文件 '{}' 为空", opts.input);
                    return Ok(());
                }
                let content = String::from_utf8_lossy(&data);
                let trimmed = content.trim();
                if trimmed.is_empty() {
                    eprintln!("错误：文件 '{}' 不包含有效内容", opts.input);
                    return Ok(());
                }
                trimmed.to_string()
            }
            Err(e) => {
                eprintln!("读取文件 '{}' 时出错：{}", opts.input, e);
                return Ok(());
            }
        }
    };

    let decoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.decode(&input_str),
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(&input_str),
    };

    let decoded = match decoded {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Base64解码失败：输入内容：{}\n错误详情：{}", input_str, e);
            return Ok(());
        }
    };

    // 智能处理解码后的数据
    match String::from_utf8(decoded.clone()) {
        std::result::Result::Ok(text) => {
            println!("{}", text);
        }
        Err(e) => {
            eprintln!(
                "Base64解码后的数据不是有效的UTF-8文本，无法输出为文本。错误详情：{}",
                e
            );
            return Ok(());
        }
    }
    Ok(())
}
