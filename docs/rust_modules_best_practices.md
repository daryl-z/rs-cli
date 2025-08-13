# Rust 模块系统最佳实践指南

## 1. 模块系统基础概念

### 模块层次结构
- **Crate**: 整个包的根，对应 `lib.rs` 或 `main.rs`
- **Module**: 代码组织的基本单位，可以是文件或目录
- **Item**: 模块中的具体项目（函数、结构体、枚举等）

### 路径前缀含义
```rust
crate::path    // 从当前 crate 根开始的绝对路径
self::path     // 从当前模块开始的相对路径
super::path    // 从父模块开始的相对路径
path           // 在当前作用域中查找
```

## 2. 模块文件组织方式

### 推荐的项目结构
```
src/
├── lib.rs          // 库的根模块，统一对外接口
├── main.rs         // 二进制入口点
├── cli/            // CLI 相关模块
│   ├── mod.rs      // cli 模块的根
│   ├── base64.rs   // base64 子模块
│   ├── csv.rs      // csv 子模块
│   └── ...
└── process/        // 处理逻辑模块
    ├── mod.rs      // process 模块的根
    ├── b64.rs      // base64 处理
    └── ...
```

## 3. 最佳实践规则

### 3.1 模块声明和导入
```rust
// ✅ 在 mod.rs 中声明子模块
mod base64;
mod csv;

// ✅ 统一重新导出，提供清晰的公共接口
pub use base64::{Base64SubCommand, Base64Format};
pub use csv::{CsvOpts, OutputFormat};
```

### 3.2 lib.rs 作为统一接口
```rust
// ✅ lib.rs 应该是库的唯一公共接口
mod cli;
mod process;

// 重新导出公共类型和函数
pub use crate::cli::{Opts, SubCommand, ...};
pub use crate::process::{process_csv, process_encode, ...};
```

### 3.3 main.rs 保持简洁
```rust
// ✅ 从库中导入所需项目
use rs_cli::{
    process_csv, process_decode, Base64SubCommand,
    Opts, SubCommand, TextSubCommand,
};

// ❌ 避免在 main.rs 中重新导出
// pub use rs_cli::Base64SubCommand;  // 不必要
```

## 4. 导入策略选择

### 4.1 什么时候使用 `crate::`
```rust
// ✅ 从当前 crate 根导入，路径明确
use crate::cli::base64::Base64Format;
use crate::process::csv_convert::process_csv;
```

### 4.2 什么时候使用 `self::`
```rust
// ✅ 导入同级模块，但现代 Rust 中通常省略
pub use self::base64::Base64Format;
// 等同于
pub use base64::Base64Format;
```

### 4.3 什么时候省略前缀
```rust
// ✅ 导入子模块时可以省略 self::
pub use base64::{Base64SubCommand, Base64Format};
pub use csv::{CsvOpts, OutputFormat};
```

## 5. 重新导出 (Re-export) 策略

### 5.1 库的分层导出
```rust
// 在 process/mod.rs 中
pub use b64::{process_decode, process_encode};
pub use csv_convert::process_csv;

// 在 lib.rs 中
pub use crate::process::{process_csv, process_decode, process_encode};
```

### 5.2 避免过度嵌套
```rust
// ❌ 避免这样的深层嵌套导入
use crate::cli::base64::format::Base64Format;

// ✅ 通过重新导出简化
// 在 cli/mod.rs 中
pub use base64::Base64Format;

// 在使用处
use crate::cli::Base64Format;
```

## 6. 项目重构前后对比

### 重构前的问题
```rust
// main.rs - 不必要的重新导出
pub use rs_cli::Base64SubCommand;
pub use rs_cli::TextSubCommand;

// cli/mod.rs - 混合使用 self:: 和直接导入
pub use self::{
    base64::Base64DecodeOpts,
    csv::CsvOpts,
};

// process/mod.rs - 注释掉的导出
// pub use b64::{process_decode, process_encode};
```

### 重构后的改进
```rust
// main.rs - 统一从库导入
use rs_cli::{
    process_csv, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
};

// cli/mod.rs - 一致的导入风格
pub use base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand};
pub use csv::{CsvOpts, OutputFormat};

// process/mod.rs - 明确的重新导出
pub use b64::{process_decode, process_encode};
pub use csv_convert::process_csv;
```

## 7. 常见错误和陷阱

### 7.1 循环依赖
```rust
// ❌ 避免模块间的循环依赖
// a.rs
use crate::b::SomeType;

// b.rs
use crate::a::AnotherType;  // 可能导致循环依赖
```

### 7.2 过度重新导出
```rust
// ❌ 避免在每个层级都重新导出所有内容
pub use everything::*;  // 污染命名空间
```

### 7.3 可见性问题
```rust
// ❌ 忘记添加 pub
mod private_module;  // 外部无法访问

// ✅ 正确的可见性声明
pub mod public_module;
pub use public_module::PublicType;
```

## 8. 性能考虑

- **重新导出不会增加运行时开销**：`pub use` 只是创建别名，不会复制代码
- **编译时间影响最小**：适当的模块组织有助于并行编译
- **二进制大小**：未使用的导出项会被编译器优化掉

## 9. 总结

遵循这些最佳实践可以让你的 Rust 项目：
- **更易维护**：清晰的模块结构和导入关系
- **更易扩展**：统一的接口设计便于添加新功能
- **更易理解**：一致的导入风格降低认知负担
- **更好的 API**：通过重新导出提供简洁的公共接口

记住：模块系统的目标是让代码更有组织性和可维护性，而不是展示技术复杂性。保持简单、一致的原则是关键。
