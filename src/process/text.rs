use crate::cli::{GenPassOpts, TextSignFormat};
use crate::process_genpass;

use rand::RngCore;

use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::{collections::HashMap, io::Read};
#[cfg(test)]
use std::{fs, path::Path};

pub trait KeyGenerator {
    fn generate() -> Result<HashMap<&'static str, Vec<u8>>>;
}

trait TextSigner {
    /// Sign the data from the reader using the provided key and format.
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let opts = GenPassOpts::get_default_opts();
        let key = process_genpass(&opts)?;
        let key = key.as_bytes().to_vec();
        Ok(HashMap::from([("blake3.txt", key)]))
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        // rand` 0.9 uses `rand_core` 0.9
        // ed25519-dalek 3.0.0-pre.0 is compatible with rand_core 0.9
        // Generate 32 random bytes for the private key
        let mut private_key_bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut private_key_bytes);

        let sk = SigningKey::from_bytes(&private_key_bytes);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();

        Ok(HashMap::from([("ed25519.sk", sk), ("ed25519.pk", pk)]))
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
        // Take only the first 32 bytes, ignoring any trailing characters (like newlines)
        let key_bytes = key
            .get(..32)
            .ok_or_else(|| anyhow::anyhow!("Key must be at least 32 bytes"))?;
        let key = SigningKey::from_bytes(key_bytes.try_into()?);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }

    #[cfg(test)]
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
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
        // Take only the first 32 bytes, ignoring any trailing characters (like newlines)
        let key_bytes = key
            .get(..32)
            .ok_or_else(|| anyhow::anyhow!("Key must be at least 32 bytes"))?;
        let key = VerifyingKey::from_bytes(key_bytes.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }

    #[cfg(test)]
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
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

    #[cfg(test)]
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // blake3::hash(&buf).as_bytes() == sig temporary value which is freed while still in use
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };

    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verifier.verify(reader, sig)
}

pub fn process_text_key_generate(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;

        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..])?;
        assert!(blake3.verify(&mut &data[..], &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let data = b"hello world";
        let sig = sk.sign(&mut &data[..])?;
        assert!(pk.verify(&mut &data[..], &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_with_trailing_newline() -> Result<()> {
        // Test that keys with trailing newlines are handled correctly
        // Generate a proper key pair
        let mut private_key_bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut private_key_bytes);

        let sk = SigningKey::from_bytes(&private_key_bytes);
        let pk = sk.verifying_key();

        // Add trailing newlines to both keys
        let mut sk_with_newline = sk.to_bytes().to_vec();
        sk_with_newline.push(b'\n');

        let mut pk_with_newline = pk.to_bytes().to_vec();
        pk_with_newline.push(b'\n');

        let signer = Ed25519Signer::try_new(&sk_with_newline)?;
        let verifier = Ed25519Verifier::try_new(&pk_with_newline)?;

        let data = b"test data";
        let sig = signer.sign(&mut &data[..])?;
        assert!(verifier.verify(&mut &data[..], &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_with_extra_bytes() -> Result<()> {
        // Test that keys with extra bytes beyond 32 are truncated correctly
        // Generate a proper key pair
        let mut private_key_bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut private_key_bytes);

        let sk = SigningKey::from_bytes(&private_key_bytes);
        let pk = sk.verifying_key();

        // Add extra bytes to both keys
        let mut sk_with_extra = sk.to_bytes().to_vec();
        sk_with_extra.extend_from_slice(&[0xff; 8]); // Add 8 extra bytes

        let mut pk_with_extra = pk.to_bytes().to_vec();
        pk_with_extra.extend_from_slice(&[0xff; 8]); // Add 8 extra bytes

        let signer = Ed25519Signer::try_new(&sk_with_extra)?;
        let verifier = Ed25519Verifier::try_new(&pk_with_extra)?;

        let data = b"test data";
        let sig = signer.sign(&mut &data[..])?;
        assert!(verifier.verify(&mut &data[..], &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_key_too_short() {
        // Test that keys shorter than 32 bytes fail appropriately
        let short_key = vec![0u8; 20];

        let signer_result = Ed25519Signer::try_new(&short_key);
        assert!(signer_result.is_err());
        if let Err(e) = signer_result {
            assert!(e.to_string().contains("Key must be at least 32 bytes"));
        }

        let verifier_result = Ed25519Verifier::try_new(&short_key);
        assert!(verifier_result.is_err());
        if let Err(e) = verifier_result {
            assert!(e.to_string().contains("Key must be at least 32 bytes"));
        }
    }

    #[test]
    fn test_blake3_with_extra_bytes() -> Result<()> {
        // Test that Blake3 keys handle extra bytes correctly
        let mut key_with_extra = vec![0u8; 40];
        #[allow(clippy::needless_range_loop)]
        for i in 0..40 {
            key_with_extra[i] = i as u8;
        }

        let blake3 = Blake3::try_new(&key_with_extra)?;

        let data = b"test data";
        let sig = blake3.sign(&mut &data[..])?;
        assert!(blake3.verify(&mut &data[..], &sig)?);
        Ok(())
    }
}
