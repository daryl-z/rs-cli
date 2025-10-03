# Repository Guidelines

## Project Structure & Module Organization
`src/main.rs` bootstraps Clap and delegates to async executors. `src/lib.rs` re-exports `cli`, `process`, and `utils` so subcommands can be reused from tests or downstream crates. Command-line definitions for CSV, HTTP, password, Base64, text (sign/encrypt with `--text`/`--cipher` overrides), and JWT operations live under `src/cli/`; corresponding business logic lives in `src/process/` and shares helpers through `src/utils.rs`. Reference datasets sit in `assets/`, deterministic signing fixtures in `fixtures/`, and supplementary design notes in `docs/`.

## Build, Test, and Development Commands
- `cargo run -- <subcommand>` — exercise the CLI, e.g. `cargo run -- csv -i assets/juventus.csv -o output.json`. JWT signing/verifying flows expect a shared secret supplied via `--secret` or the `JWT_SECRET` env before release. ChaCha20-Poly1305 encrypt/decrypt requires reusing the Base64 output (nonce+ciphertext) with the same key; `--text`/`--cipher` let you bypass stdin when scripting.
- `cargo build --release` — produce `target/release/rst` for distribution.
- `cargo fmt --all` — enforce repository-wide formatting.
- `cargo clippy --all-targets --all-features -D warnings` — keep lints clean.
- `cargo test --all-features` — run unit and async tests.
- `cargo deny check` — optional dependency audit driven by `deny.toml` before releases.

## Coding Style & Naming Conventions
Honor rustfmt defaults (4-space indent, trailing commas) and avoid manual formatting. Use `snake_case` for modules/files, `PascalCase` for types, and `kebab-case` for CLI flags to match Clap derivations. Structure fallible APIs around `anyhow::Result` and emit diagnostics with `tracing` macros. Keep validation and help strings consistent with existing tone, including the localized messages in `src/cli.rs`.

## Testing Guidelines
Add tests beside the code under `#[cfg(test)]` modules, following the `test_*` naming already in place. Prefer `#[tokio::test]` for async flows like HTTP serving so futures can be awaited without boilerplate. Pull fixtures from `assets/` and `fixtures/` instead of inventing ad-hoc data; document any new fixture in that directory.

## Commit & Pull Request Guidelines
Match the repository’s Conventional Commit leaning (`feat:`, `fix:`, `refactor:`) and keep messages in the imperative mood. Pull requests should summarize user-facing impact, link issues when available, and list the local commands you ran (fmt, clippy, test, targeted `cargo run`). Include screenshots or sample CLI output if behavior changes. Request review only after linting and tests succeed.

## Security & Configuration Tips
Never commit real secrets; the Ed25519 material in `fixtures/` is sample data only. Configure JWT secrets through `JWT_SECRET` or config files before shipping; the CLI refuses to operate without a supplied secret so avoid committing sample values. Treat ChaCha20-Poly1305 keys as sensitive and store the Base64 output securely (the nonce is prefixed to enable decryption); prefer piping secrets instead of recording them in shell history. When serving directories, run with `RUST_LOG=info` to monitor access logs and verify paths; directory listings are now generated automatically when no file is found. Use `cargo deny check bans licenses sources` if supply-chain compliance matters for a release.
