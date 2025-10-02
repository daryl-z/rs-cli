use crate::cli::{JwtSignOpts, JwtVerifyOpts};
use anyhow::{ensure, Context, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject
    aud: String, // Audience
    exp: u64,    // Expiration time
    iat: u64,    // Issued at
}

const MIN_SECRET_LEN: usize = 32;

fn resolve_secret(secret: &Option<String>) -> Result<Vec<u8>> {
    if let Some(ref value) = secret {
        ensure!(!value.is_empty(), "Secret cannot be empty");
        ensure!(
            value.len() >= MIN_SECRET_LEN,
            "Secret must be at least {} bytes for HS256",
            MIN_SECRET_LEN
        );
        return Ok(value.as_bytes().to_vec());
    }

    let env_secret = env::var("JWT_SECRET")
        .context("HS256 secret missing; supply --secret or set JWT_SECRET env var")?;
    ensure!(!env_secret.is_empty(), "JWT_SECRET cannot be empty");
    ensure!(
        env_secret.len() >= MIN_SECRET_LEN,
        "JWT_SECRET must be at least {} bytes for HS256",
        MIN_SECRET_LEN
    );
    Ok(env_secret.into_bytes())
}

/// Parse duration string like "14d", "1h", "30m" into seconds
fn parse_duration(duration_str: &str) -> Result<u64> {
    let duration_str = duration_str.trim();
    if duration_str.is_empty() {
        return Err(anyhow::anyhow!("Duration cannot be empty"));
    }

    let (number_part, unit_part) =
        if let Some(pos) = duration_str.chars().position(|c| !c.is_ascii_digit()) {
            duration_str.split_at(pos)
        } else {
            return Err(anyhow::anyhow!("Duration must include a unit (d, h, m, s)"));
        };

    let number: u64 = number_part
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid number in duration: {}", number_part))?;

    let seconds = match unit_part {
        "s" => number,
        "m" => number * 60,
        "h" => number * 60 * 60,
        "d" => number * 60 * 60 * 24,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid duration unit: {}. Use s, m, h, or d",
                unit_part
            ))
        }
    };

    Ok(seconds)
}

/// Get current timestamp in seconds since Unix epoch
fn get_current_timestamp() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|e| anyhow::anyhow!("Failed to get current time: {}", e))
}

/// Sign a JWT token with the given options
pub fn process_jwt_sign(opts: &JwtSignOpts) -> Result<String> {
    let now = get_current_timestamp()?;
    let exp_duration = parse_duration(&opts.exp)?;
    let secret = resolve_secret(&opts.secret)?;

    let claims = Claims {
        sub: opts.sub.clone(),
        aud: opts.aud.clone(),
        exp: now + exp_duration,
        iat: now,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(&secret);

    encode(&header, &claims, &encoding_key)
        .map_err(|e| anyhow::anyhow!("Failed to encode JWT: {}", e))
}

/// Verify a JWT token
pub fn process_jwt_verify(opts: &JwtVerifyOpts) -> Result<bool> {
    let secret = resolve_secret(&opts.secret)?;
    let decoding_key = DecodingKey::from_secret(&secret);
    let mut validation = Validation::new(Algorithm::HS256);
    let aud_slice = [opts.aud.as_str()];
    validation.set_audience(&aud_slice);

    match decode::<Claims>(&opts.token, &decoding_key, &validation) {
        Ok(token_data) => {
            tracing::info!(
                sub = %token_data.claims.sub,
                aud = %token_data.claims.aud,
                exp = token_data.claims.exp,
                "jwt_verified"
            );
            Ok(true)
        }
        Err(e) => {
            tracing::warn!(error = %e, "jwt_verification_failed");
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const SECRET: &str = "averylongandsecurejwtsecretstring!!!";

    #[test]
    fn test_parse_duration() -> Result<()> {
        assert_eq!(parse_duration("30s")?, 30);
        assert_eq!(parse_duration("5m")?, 300);
        assert_eq!(parse_duration("2h")?, 7200);
        assert_eq!(parse_duration("1d")?, 86400);
        assert_eq!(parse_duration("14d")?, 1_209_600);
        Ok(())
    }

    #[test]
    fn test_parse_duration_invalid() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("30").is_err());
        assert!(parse_duration("30x").is_err());
        assert!(parse_duration("abc").is_err());
    }

    #[test]
    fn test_jwt_sign_and_verify() -> Result<()> {
        let sign_opts = JwtSignOpts {
            sub: "acme".to_string(),
            aud: "device1".to_string(),
            exp: "1h".to_string(),
            secret: Some(SECRET.to_string()),
        };

        let token = process_jwt_sign(&sign_opts)?;
        assert!(!token.is_empty());

        let verify_opts = JwtVerifyOpts {
            token: token.clone(),
            aud: "device1".to_string(),
            secret: Some(SECRET.to_string()),
        };

        let is_valid = process_jwt_verify(&verify_opts)?;
        assert!(is_valid);
        Ok(())
    }

    #[test]
    fn test_jwt_verify_invalid_token() -> Result<()> {
        let verify_opts = JwtVerifyOpts {
            token: "invalid.jwt.token".to_string(),
            aud: "device1".to_string(),
            secret: Some(SECRET.to_string()),
        };

        let is_valid = process_jwt_verify(&verify_opts)?;
        assert!(!is_valid);
        Ok(())
    }

    #[test]
    fn test_jwt_verify_malformed_token() -> Result<()> {
        let verify_opts = JwtVerifyOpts {
            token: "not-a-jwt-at-all".to_string(),
            aud: "device1".to_string(),
            secret: Some(SECRET.to_string()),
        };

        let is_valid = process_jwt_verify(&verify_opts)?;
        assert!(!is_valid);
        Ok(())
    }

    #[test]
    fn test_resolve_secret_requires_length() {
        let short = Some("short".to_string());
        assert!(resolve_secret(&short).is_err());
    }

    #[test]
    fn test_resolve_secret_env_fallback() -> Result<()> {
        let key = "averylongandsecurejwtsecretstring!!!";
        let prev = env::var("JWT_SECRET").ok();
        env::set_var("JWT_SECRET", key);
        let resolved = resolve_secret(&None)?;
        assert_eq!(resolved, key.as_bytes());
        if let Some(prev) = prev {
            env::set_var("JWT_SECRET", prev);
        } else {
            env::remove_var("JWT_SECRET");
        }
        Ok(())
    }
}
