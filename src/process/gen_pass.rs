use crate::opts::GenPassOpts;
use rand::seq::{IndexedRandom, SliceRandom};
use std::iter;
use zxcvbn::zxcvbn;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*().<>?_+-=/";

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::rng();

    let char_sets = [
        (opts.lowercase, LOWERCASE),
        (opts.uppercase, UPPERCASE),
        (opts.numbers, NUMBERS),
        (opts.symbols, SYMBOLS),
    ];
    let enabled_sets: Vec<&[u8]> = char_sets
        .into_iter() // 使用 into_iter 消费数组
        .filter(|(enabled, _)| *enabled)
        .map(|(_, set)| set)
        .collect();

    // 3. 在所有操作开始前，首先处理边界情况
    if enabled_sets.is_empty() {
        eprintln!("错误：请至少选择一个字符集！");
        return Ok(());
    }

    // 4. 函数式地生成“保证存在的字符”
    let mut password: Vec<u8> = enabled_sets
        .iter()
        .map(|set| *set.choose(&mut rng).unwrap())
        .collect();

    let charset: Vec<u8> = enabled_sets.iter().flat_map(|set| *set).cloned().collect();

    let remaining_len = opts.length.saturating_sub(password.len());

    // 使用 chain 将保证字符和填充字符的迭代器连接起来，逻辑更连贯
    if remaining_len > 0 {
        let filler_chars =
            iter::repeat_with(|| *charset.choose(&mut rng).unwrap()).take(remaining_len);
        password.extend(filler_chars);
    }

    password.shuffle(&mut rng);
    let final_password = String::from_utf8(password)?;
    println!("{}", final_password);
    let score = zxcvbn(&final_password, &[]).score();
    eprintln!("密码强度评分：{} / 5", score);

    Ok(())
}
