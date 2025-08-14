use crate::cli::{GenPassOpts, TextSignFormat, TextSignOpts, TextVerifyOpts};
use crate::process_genpass;
use crate::utils::get_reader;

// use rand::rand_core::OsRng;

use anyhow::Result;
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::{fs, io::Read, path::Path};

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

trait TextSign {
    // 动态分发
    /// Sign the data from the reader using the provided key and format.
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
// 静态分发 impl Read 等效 reader:R
pub trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized; // 返回有固定长度的数据结构 不是str [u8]
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(&path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(&path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(&path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let opts = GenPassOpts::get_default_opts();
        let key = process_genpass(&opts)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        // rand` 0.9 uses `rand_core` 0.9
        // - `ed25519-dalek` 2.2.0 requires `rand_core` 0.6
        // let mut csprng = OsRng;
        // let sk: SigningKey = SigningKey::generate(&mut csprng);
        // let pk = sk.verifying_key().to_bytes().to_vec();
        // let sk = sk.to_bytes().to_vec();

        Ok(vec![])
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}

// Blake3
struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Blake3 { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // blake3::hash(&buf).as_bytes() == sig temporary value which is freed while still in use
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub fn process_text_sign(opts: &TextSignOpts) -> Result<()> {
    // let signer = Blake3 { key: [0; 32] };
    let mut reader = get_reader(&opts.input)?;

    let signed = match &opts.format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(&opts.key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(&opts.key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = BASE64_URL_SAFE_NO_PAD.encode(&signed);
    println!("{}", signed);
    Ok(())
}

pub fn process_text_verify(opts: &TextVerifyOpts) -> Result<()> {
    let mut reader = get_reader(&opts.input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let sig = BASE64_URL_SAFE_NO_PAD.decode(&opts.sig)?;

    let verified = match &opts.format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(&opts.key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(&opts.key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };
    println!("{}", verified);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;

        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..])?;
        assert!(blake3.verify(&data[..], &sig)?);
        Ok(())
    }
}
