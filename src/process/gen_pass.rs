use crate::opts::GenPassOpts;
use rand::Rng;

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut chars = Vec::new();
    if opts.lowercase {
        chars.push("abcdefghijklmnopqrstuvwxyz");
    }
    if opts.uppercase {
        chars.push("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if opts.numbers {
        chars.push("0123456789");
    }
    if opts.symbols {
        chars.push("!@#$%^&*().<>?_+-=/");
    }
    let password = (0..opts.length)
        .map(|_| {
            let idx = rng.random_range(0..chars.len());
            let char_set = &chars[idx];
            let idx = rng.random_range(0..char_set.len());
            char_set.chars().nth(idx).unwrap()
        })
        .collect::<String>();

    println!("{}", password);
    Ok(())
}
