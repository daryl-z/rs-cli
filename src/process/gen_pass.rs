use crate::opts::GenPassOpts;
use rand::seq::{IndexedRandom, SliceRandom};
use std::iter;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*().<>?_+-=/";

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::rng();

    // 1. 声明式地定义所有可能的字符集及其启用条件
    let char_sets = [
        (opts.lowercase, LOWERCASE),
        (opts.uppercase, UPPERCASE),
        (opts.numbers, NUMBERS),
        (opts.symbols, SYMBOLS),
    ];

    // 2. 使用迭代器链构建基础密码和完整字符池
    let mut password: Vec<u8> = Vec::new();
    let mut charset: Vec<u8> = Vec::new();

    char_sets
        .iter()
        .filter(|(enabled, _)| *enabled)
        .for_each(|(_, set)| {
            // a. 确保每种启用的字符集都至少有一个字符进入密码
            password.push(*set.choose(&mut rng).unwrap());
            // b. 将该字符集添加到总的字符池中
            charset.extend_from_slice(set);
        });

    // 如果没有选择任何字符集，则提前返回
    if charset.is_empty() {
        eprintln!("错误：请至少选择一个字符集！");
        return Ok(());
    }

    // 3. 使用函数式方法生成剩余的填充字符
    // 一个有“地板”的减法。结果最小只能到达地板（0），不能再低了
    let remaining_len = opts.length.saturating_sub(password.len());

    let filler_chars = iter::repeat_with(|| *charset.choose(&mut rng).unwrap()).take(remaining_len);

    // 4. 将填充字符追加到密码中
    password.extend(filler_chars);

    // 5. 打乱最终密码
    password.shuffle(&mut rng);

    let final_password = String::from_utf8(password)?;
    println!("{}", final_password);

    Ok(())
}
