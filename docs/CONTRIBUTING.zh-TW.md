# 為 BOM 計算引擎做出貢獻

[English](../CONTRIBUTING.md) | [简体中文](./CONTRIBUTING.zh-CN.md) | [Deutsch](./CONTRIBUTING.de.md)

感謝您有興趣為本項目做出貢獻！我們歡迎來自社區的貢獻。

## 🌍 語言

本項目支持多種語言：
- English（主要開發語言）
- 繁體中文（Traditional Chinese）
- 简体中文（Simplified Chinese）
- Deutsch（German）

文檔和討論可以使用這些語言中的任何一種。

## 🚀 入門指南

1. Fork 此倉庫
2. 克隆您的 fork：`git clone https://github.com/yourname/bom.git`
3. 創建分支：`git checkout -b feature/your-feature-name`
4. 進行修改
5. 運行測試：`cargo test`
6. 提交變更：`git commit -am 'Add some feature'`
7. 推送到分支：`git push origin feature/your-feature-name`
8. 創建 Pull Request

## 📝 開發指南

### 代碼風格

- 遵循 Rust 官方風格指南
- 提交前運行 `cargo fmt`
- 運行 `cargo clippy` 並修復警告
- 為公共 API 編寫文檔
- 為新功能添加測試

### 提交訊息

使用清晰、描述性的提交訊息：

```
feat: add SAP adapter for BOM repository
fix: correct cost calculation for phantom components
docs: update README with new examples
test: add benchmarks for deep BOM structures
```

### 測試

- 為新函數編寫單元測試
- 為新功能添加整合測試
- 確保所有測試通過：`cargo test --all`
- 運行基準測試以檢查性能：`cargo bench`

### 文檔

- 添加新功能時更新 README
- 為公共 API 添加文檔註釋
- 更新 CHANGELOG.md
- 將重要文檔翻譯成支持的語言

## 🐛 報告錯誤

報告錯誤時，請包含：

1. 您的操作系統和 Rust 版本
2. 重現錯誤的步驟
3. 預期行為
4. 實際行為
5. 錯誤訊息或日誌

## 💡 建議功能

我們喜歡新想法！建議功能時：

1. 檢查是否已經有人建議過
2. 解釋使用案例
3. 描述建議的解決方案
4. 考慮向後兼容性

## 🔍 代碼審查流程

1. 所有提交都需要審查
2. 我們將在幾天內審查您的 PR
3. 處理任何反饋
4. 一旦批准，我們將合併您的 PR

## 📜 授權

通過貢獻，您同意您的貢獻將根據以下許可證授權：
- AGPL-3.0（用於開源使用）
- 商業許可證（由項目維護者決定）

## 🙏 致謝

貢獻者將在以下地方得到認可：
- CONTRIBUTORS.md
- 發布說明
- 項目文檔

## 💬 溝通

- **GitHub Issues**：錯誤報告和功能請求
- **GitHub Discussions**：一般問題和討論
- **Email**：xiaoivan1@proton.me（私人諮詢）

## 🎯 優先領域

我們特別歡迎以下方面的貢獻：

1. **ERP/PLM 適配器**
   - SAP 連接器
   - Oracle 連接器
   - 其他 ERP 系統

2. **性能改進**
   - 演算法優化
   - 緩存增強
   - 並行處理改進

3. **語言綁定**
   - Python 綁定
   - Java 綁定
   - .NET 綁定

4. **文檔**
   - 使用範例
   - 整合指南
   - 翻譯

5. **測試**
   - 更多測試案例
   - 性能基準測試
   - 真實世界場景

## 📊 開發環境設置

### 前置要求

```bash
# 安裝 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安裝開發工具
cargo install cargo-watch
cargo install cargo-edit
```

### 構建

```bash
# 構建所有 crates
cargo build --all

# 使用優化構建
cargo build --release

# 構建 FFI 庫
cargo build --release -p bom-ffi
```

### 測試

```bash
# 運行所有測試
cargo test --all

# 運行特定 crate 測試
cargo test -p bom-core

# 運行基準測試
cargo bench
```

### 文檔

```bash
# 生成文檔
cargo doc --no-deps --open

# 檢查文檔
cargo doc --no-deps --all
```

## 🔧 項目結構

```
bom/
├── crates/
│   ├── bom-core/       # 核心數據模型
│   ├── bom-graph/      # 圖數據結構
│   ├── bom-calc/       # 計算引擎
│   ├── bom-cache/      # 緩存層
│   ├── bom-ffi/        # C FFI 綁定
│   ├── bom-adapters/   # ERP/PLM 適配器
│   └── bom-benches/    # 基準測試
├── examples/           # 使用範例
├── docs/              # 文檔
└── tests/             # 整合測試
```

## ❓ 有問題？

如果您有問題：

1. 檢查現有文檔
2. 搜索 GitHub Issues
3. 在 GitHub Discussions 中詢問
4. Email xiaoivan1@proton.me

感謝您的貢獻！🎉
