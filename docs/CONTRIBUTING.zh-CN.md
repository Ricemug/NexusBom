# 为 BOM 计算引擎做出贡献

[English](../CONTRIBUTING.md) | [繁體中文](./CONTRIBUTING.zh-TW.md) | [Deutsch](./CONTRIBUTING.de.md)

感谢您有兴趣为本项目做出贡献！我们欢迎来自社区的贡献。

## 🌍 语言

本项目支持多种语言：
- English（主要开发语言）
- 繁體中文（Traditional Chinese）
- 简体中文（Simplified Chinese）
- Deutsch（German）

文档和讨论可以使用这些语言中的任何一种。

## 🚀 入门指南

1. Fork 此仓库
2. 克隆您的 fork：`git clone https://github.com/yourname/bom.git`
3. 创建分支：`git checkout -b feature/your-feature-name`
4. 进行修改
5. 运行测试：`cargo test`
6. 提交变更：`git commit -am 'Add some feature'`
7. 推送到分支：`git push origin feature/your-feature-name`
8. 创建 Pull Request

## 📝 开发指南

### 代码风格

- 遵循 Rust 官方风格指南
- 提交前运行 `cargo fmt`
- 运行 `cargo clippy` 并修复警告
- 为公共 API 编写文档
- 为新功能添加测试

### 提交消息

使用清晰、描述性的提交消息：

```
feat: add SAP adapter for BOM repository
fix: correct cost calculation for phantom components
docs: update README with new examples
test: add benchmarks for deep BOM structures
```

### 测试

- 为新函数编写单元测试
- 为新功能添加集成测试
- 确保所有测试通过：`cargo test --all`
- 运行基准测试以检查性能：`cargo bench`

### 文档

- 添加新功能时更新 README
- 为公共 API 添加文档注释
- 更新 CHANGELOG.md
- 将重要文档翻译成支持的语言

## 🐛 报告错误

报告错误时，请包含：

1. 您的操作系统和 Rust 版本
2. 重现错误的步骤
3. 预期行为
4. 实际行为
5. 错误消息或日志

## 💡 建议功能

我们喜欢新想法！建议功能时：

1. 检查是否已经有人建议过
2. 解释使用案例
3. 描述建议的解决方案
4. 考虑向后兼容性

## 🔍 代码审查流程

1. 所有提交都需要审查
2. 我们将在几天内审查您的 PR
3. 处理任何反馈
4. 一旦批准，我们将合并您的 PR

## 📜 授权

通过贡献，您同意您的贡献将根据以下许可证授权：
- AGPL-3.0（用于开源使用）
- 商业许可证（由项目维护者决定）

## 🙏 致谢

贡献者将在以下地方得到认可：
- CONTRIBUTORS.md
- 发布说明
- 项目文档

## 💬 沟通

- **GitHub Issues**：错误报告和功能请求
- **GitHub Discussions**：一般问题和讨论
- **Email**：xiaoivan1@proton.me（私人咨询）

## 🎯 优先领域

我们特别欢迎以下方面的贡献：

1. **ERP/PLM 适配器**
   - SAP 连接器
   - Oracle 连接器
   - 其他 ERP 系统

2. **性能改进**
   - 算法优化
   - 缓存增强
   - 并行处理改进

3. **语言绑定**
   - Python 绑定
   - Java 绑定
   - .NET 绑定

4. **文档**
   - 使用示例
   - 集成指南
   - 翻译

5. **测试**
   - 更多测试案例
   - 性能基准测试
   - 真实世界场景

## 📊 开发环境设置

### 前置要求

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装开发工具
cargo install cargo-watch
cargo install cargo-edit
```

### 构建

```bash
# 构建所有 crates
cargo build --all

# 使用优化构建
cargo build --release

# 构建 FFI 库
cargo build --release -p bom-ffi
```

### 测试

```bash
# 运行所有测试
cargo test --all

# 运行特定 crate 测试
cargo test -p bom-core

# 运行基准测试
cargo bench
```

### 文档

```bash
# 生成文档
cargo doc --no-deps --open

# 检查文档
cargo doc --no-deps --all
```

## 🔧 项目结构

```
bom/
├── crates/
│   ├── bom-core/       # 核心数据模型
│   ├── bom-graph/      # 图数据结构
│   ├── bom-calc/       # 计算引擎
│   ├── bom-cache/      # 缓存层
│   ├── bom-ffi/        # C FFI 绑定
│   ├── bom-adapters/   # ERP/PLM 适配器
│   └── bom-benches/    # 基准测试
├── examples/           # 使用示例
├── docs/              # 文档
└── tests/             # 集成测试
```

## ❓ 有问题？

如果您有问题：

1. 检查现有文档
2. 搜索 GitHub Issues
3. 在 GitHub Discussions 中询问
4. Email xiaoivan1@proton.me

感谢您的贡献！🎉
