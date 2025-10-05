# 🏭 BOM 計算引擎

> 高效能物料清單計算引擎，專為 PLM/ERP 系統設計

[English](../README.md) | [简体中文](./README.zh-CN.md) | [Deutsch](./README.de.md)

[![授權](https://img.shields.io/badge/license-AGPL--3.0%20%7C%20Commercial-blue.svg)](../LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![測試](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

## 🚀 特色功能

- ⚡ **超高速度**：圖結構建構僅需微秒，物料展開 <25μs
- 🔧 **SAP/Oracle 相容**：為企業級 ERP 系統量身打造
- 🌐 **多語言支援**：提供 C/C++/Python/Java 的 FFI 綁定
- 💾 **智慧快取**：雙層快取（記憶體 + 持久化）達到最佳效能
- 🔄 **平行處理**：運用 Rayon 發揮多核心運算能力
- 📊 **完整 BOM 功能**：
  - 物料展開（多階 BOM）
  - 成本計算與累加
  - 反查分析（Where-Used）
  - 變更影響分析
  - 虛設料件處理
  - 替代 BOM 支援

## 📦 安裝方式

### Rust

```toml
[dependencies]
bom-core = "0.1"
bom-graph = "0.1"
bom-calc = "0.1"
```

### C/C++

```bash
# 下載預編譯執行檔
wget https://github.com/yourname/bom/releases/latest/download/libbom_ffi.so

# 或從原始碼編譯
cargo build --release -p bom-ffi
```

### Python（透過 FFI）

```python
# 即將推出
pip install bom-engine
```

## 🎯 快速開始

```rust
use bom_core::repository::memory::InMemoryRepository;
use bom_graph::BomGraph;
use bom_calc::explosion::ExplosionCalculator;

// 建立簡單的 BOM 結構
let mut repo = InMemoryRepository::new();

// 新增元件
repo.add_component(create_component("BIKE", "腳踏車", 500));
repo.add_component(create_component("FRAME", "車架", 150));
repo.add_component(create_component("WHEEL", "輪子", 50));

// 定義 BOM 關係
repo.add_bom_item(create_bom_item("BIKE", "FRAME", 1));
repo.add_bom_item(create_bom_item("BIKE", "WHEEL", 2));

// 建立圖結構並計算
let graph = BomGraph::from_component(&repo, &ComponentId::new("BIKE"), None)?;
let calculator = ExplosionCalculator::new(&graph);
let result = calculator.explode(&ComponentId::new("BIKE"), Decimal::from(10))?;

println!("需要的總項目數：{}", result.items.len());
```

完整範例請參考 [examples/simple](../examples/simple)。

## 📊 效能表現

在一般消費級硬體上的測試結果：

| 操作 | 時間 | 說明 |
|------|------|------|
| 圖結構建構 | ~1.2-2.1 μs | 建立 BOM 圖結構 |
| 物料展開 | ~19-24 μs | 多階 BOM 展開 |
| 成本計算 | ~21-28 μs | 成本累加與分析 |
| 反查查詢 | ~192 ns | 逆向 BOM 查找 |

詳細指標請參閱 [BENCHMARK_RESULTS.md](../BENCHMARK_RESULTS.md)。

## 🏗️ 架構設計

```
┌─────────────────────────────────────────────┐
│            應用層                            │
│    (ERP/PLM/客製化應用程式)                 │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│           FFI 層 (C API)                     │
│       bom-ffi (libbom_ffi.so)               │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│            計算引擎                          │
│  ┌──────────┬──────────┬──────────────┐    │
│  │物料展開  │  成本    │  反查分析    │    │
│  └──────────┴──────────┴──────────────┘    │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│         圖結構 & 快取層                      │
│  ┌─────────────┬────────────────────┐       │
│  │  BOM 圖     │  分層快取          │       │
│  │  (Arena)    │  (Moka + Redb)     │       │
│  └─────────────┴────────────────────┘       │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│          資料存取層                          │
│     (SAP/Oracle/客製化資料庫轉接器)         │
└─────────────────────────────────────────────┘
```

## 🔌 系統整合

### SAP 整合

```rust
// 使用 SAP 轉接器（即將推出）
use bom_adapters::sap::SapRepository;

let repo = SapRepository::connect(sap_config)?;
let graph = BomGraph::from_component(&repo, &component_id, None)?;
```

### Oracle 整合

```rust
// 使用 Oracle 轉接器（即將推出）
use bom_adapters::oracle::OracleRepository;

let repo = OracleRepository::connect(oracle_config)?;
let graph = BomGraph::from_component(&repo, &component_id, None)?;
```

### 自訂資料庫

實作 `BomRepository` trait：

```rust
impl BomRepository for MyCustomRepo {
    fn get_component(&self, id: &ComponentId) -> Result<Component>;
    fn get_bom_items(&self, parent_id: &ComponentId, date: Option<DateTime<Utc>>) -> Result<Vec<BomItem>>;
    // ... 其他方法
}
```

## 📚 技術文件

### API 文件

在本地生成 API 文件：

```bash
# 生成並在瀏覽器中開啟文件
cargo doc --no-deps --open

# 生成所有 crate 的文件
cargo doc --workspace --no-deps
```

文件位置：`target/doc/bom_core/index.html`

### 其他文件

- [快速入門](./QUICKSTART.zh-TW.md)
- [架構概述](./PROJECT_SUMMARY.zh-TW.md)
- [效能測試](./BENCHMARK_RESULTS.zh-TW.md)
- [貢獻指南](./CONTRIBUTING.zh-TW.md)

## 🤝 貢獻參與

我們歡迎各種形式的貢獻！請參閱 [CONTRIBUTING.md](../CONTRIBUTING.md) 了解貢獻準則。

## 💝 支持本專案

如果這個專案對您的事業有幫助，請考慮支持開發：

- ⭐ 給專案按個星星
- 🐛 回報問題並提出功能建議
- 💰 [Ko-fi 贊助](https://ko-fi.com/ivanh0906)
- 💰 [GitHub Sponsors](https://github.com/sponsors/yourname)
- 🏢 [商業授權](../COMMERCIAL-LICENSE.md) 供企業使用

## 📄 授權方式

本專案採用雙重授權：

- **開源授權**：[AGPL-3.0](../LICENSE) 供開源專案使用
- **商業授權**：[商業授權](../COMMERCIAL-LICENSE.md) 供專有軟體使用

請選擇最適合您需求的授權方式。

## 🌟 使用案例

- 您的公司名稱
- [提交 PR 加入您的案例！]

## 📞 聯絡方式

- **問題回報**：[GitHub Issues](https://github.com/yourname/bom/issues)
- **討論交流**：[GitHub Discussions](https://github.com/yourname/bom/discussions)
- **電子郵件**：xiaoivan1@proton.me

---

為製造業用 ❤️ 打造
