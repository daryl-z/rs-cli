# RS-CLI

## Overview / 项目概述
- RS-CLI is a Rust-powered toolbox that bundles CSV conversion, Base64 utilities, password generation, text signing, JWT handling, and a lightweight static HTTP server in one binary named `rst`.
- RS-CLI 是一个由 Rust 编写的多功能命令行工具，通过单一的 `rst` 可执行文件提供 CSV 转换、Base64 编解码、密码生成、文本与 JWT 签名/验证能力以及轻量级静态 HTTP 服务等能力。

## Quick Start / 快速开始
- Install a recent stable Rust toolchain (edition 2021). Build locally with `cargo build --release` and grab the binary from `target/release/rst`.
- 安装最新稳定版 Rust（支持 2021 edition），执行 `cargo build --release` 后即可在 `target/release/rst` 获取可执行文件。
- Run `cargo run -- --help` for the global help menu, or append a subcommand (e.g., `cargo run -- csv --help`).
- 运行 `cargo run -- --help` 查看全局帮助，也可以追加子命令（例如 `cargo run -- csv --help`）。

## Command Reference / 命令参考
| Command | English Description | 中文说明 |
| --- | --- | --- |
| `rst csv -i <path> [-o output] [--format json|yaml|toml] [--delimiter ,] [--header/--no-header]` | Convert CSV into JSON/YAML/TOML, honoring custom delimiters and header rows. Output defaults to `output.<format>`. | 将 CSV 转换为 JSON/YAML/TOML，可自定义分隔符及是否存在表头；若未指定输出文件，默认生成 `output.<format>`。 |
| `rst genpass [--length N] [--uppercase/--no-uppercase] [--lowercase/--no-lowercase] [--numbers/--no-numbers] [--symbols/--no-symbols]` | Generate a random password and print a strength score sourced from `zxcvbn`. | 生成随机密码并借助 `zxcvbn` 输出强度评分。 |
| `rst base64 encode|decode [-i <file>|-] [--format standard|urlsafe]` | Stream Base64 encoding or decoding from stdin or file with standard or URL-safe alphabets. | 以标准或 URL 安全字母表对标准输入或文件进行 Base64 编码/解码。 |
| `rst text sign --input <file|-> --key <keyfile> [--format blake3|ed25519]` | Produce a signature in URL-safe Base64 for text payloads using Blake3 MAC or Ed25519. | 使用 Blake3 MAC 或 Ed25519 为文本生成签名，并以 URL 安全 Base64 输出。 |
| `rst text verify --input <file|-> --key <keyfile> --sig <base64>` | Validate a signature and print `true`/`false`. | 校验签名并输出 `true`/`false`。 |
| `rst text generate --format blake3|ed25519 --output <dir>` | Create Blake3 secret keys or Ed25519 key pairs in the target directory. | 在目标目录生成 Blake3 密钥或 Ed25519 密钥对。 |
| `rst http serve [--dir <path>] [--port <u16>]` | Host static files from a directory via Axum on the given port (default 8080). | 基于 Axum 在指定端口（默认 8080）托管某目录下的静态文件。 |
| `rst jwt sign --sub <subject> --aud <audience> [--exp 14d] [--secret <secret>]` | Issue an HS256 token with the provided subject, audience, and TTL (default 14 days). Provide the signing key via --secret or the JWT_SECRET env var. | 基于 HS256 签发带有主体、受众及有效期（默认 14 天）的 JWT，密钥可通过 --secret 或 JWT_SECRET 环境变量提供。 |
| `rst jwt verify --token <jwt> --aud <audience> [--secret <secret>]` | Validate an HS256 token using the shared secret (flag or JWT_SECRET env) and enforce the expected audience. | 校验使用共享密钥签名的 HS256 JWT，并在提供的受众匹配时返回结果；密钥可来自 --secret 或 JWT_SECRET 环境变量。 |

> Tip 提示：Use `-` for `--input` or `--key` to read from stdin in both the Base64 and Text flows. / 在 Base64 与文本子命令中，`--input` 或 `--key` 传入 `-` 可从标准输入读取。
> Note 提示：Set `JWT_SECRET` or pass `--secret` when running JWT subcommands to provide the HS256 key. / 运行 JWT 子命令时请通过 `JWT_SECRET` 环境变量或 `--secret` 参数提供 HS256 密钥。

## Dependency Highlights / 依赖说明
| Crate | English Purpose | 中文用途 |
| --- | --- | --- |
| `clap` + `enum_dispatch` | Declarative CLI parsing with subcommand trait dispatch. | 用于声明式命令行解析并通过枚举分发子命令行为。 |
| `tokio` | Async runtime backing HTTP serving and IO-heavy workflows. | 为 HTTP 服务与 IO 操作提供异步运行时。 |
| `axum` + `tower-http` | Serve static directories with routing, compression, and tracing middleware. | 为静态目录提供路由、压缩与日志中间件。 |
| `serde`, `serde_json`, `serde_yaml`, `toml` | Serialize structured CSV data into multiple formats. | 将结构化 CSV 数据序列化为多种格式。 |
| `csv` | Parse delimited files while preserving headers. | 解析带表头的分隔文本文件。 |
| `base64`, `blake3`, `ed25519-dalek`, `rand`, `zxcvbn` | Power cryptographic utilities, entropy generation, and password scoring. | 支撑加密工具、随机数生成与密码强度评分。 |
| `jsonwebtoken` | HS256 JWT signing and verification helpers. | 提供 HS256 JWT 的签名与验证功能。 |
| `anyhow`, `tracing`, `tracing-subscriber` | Provide ergonomic error handling and structured logging. | 提供简洁的错误处理与结构化日志。 |
| `tempfile` (dev) | Support isolated filesystem tests. | 为文件系统相关测试提供隔离环境。 |

## Development Notes / 开发说明
- Run `cargo fmt --all`, `cargo clippy --all-targets --all-features -D warnings`, and `cargo test --all-features` before submitting patches.
- 在提交补丁前请依次运行 `cargo fmt --all`、`cargo clippy --all-targets --all-features -D warnings` 与 `cargo test --all-features`。
- Use `cargo deny check` when auditing dependency licenses or security advisories.
- 进行依赖或许可证审核时执行 `cargo deny check`。

## License / 许可证
- Distributed under the MIT License as declared in `Cargo.toml`.
- 本项目依据 `Cargo.toml` 中声明的 MIT 许可证发布。
