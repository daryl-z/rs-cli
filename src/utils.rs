use anyhow::Result;
use std::fs::File;
use std::io::{self, Read};

/// 根据输入参数获取相应的 Reader
///
/// # Arguments
/// * `input` - 输入源，"-" 表示标准输入，否则为文件路径
///
/// # Returns
/// * `Result<Box<dyn Read>>` - 返回一个实现了 Read trait 的 Box
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(io::stdin()))
    } else {
        let file = File::open(input)?;
        Ok(Box::new(file))
    }
}

/// 获取输入的字节数据
///
/// # Arguments
/// * `input` - 输入源，"-" 表示标准输入，否则为文件路径
///
/// # Returns
/// * `Result<Vec<u8>>` - 读取的字节数据
pub fn get_input_bytes(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    Ok(data)
}

/// 获取输入的字符串数据
///
/// # Arguments
/// * `input` - 输入源，"-" 表示标准输入，否则为文件路径
///
/// # Returns
/// * `Result<String>` - 读取的字符串数据
pub fn get_input_string(input: &str) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    Ok(data)
}
