# BOM - Bill of Materials Library in Rust

高性能 BOM（Bill of Materials，物料清單）計算引擎，使用 Rust 實現。

## 專案目標

打造一個可嵌入各家 PLM 或 ERP 系統的 BOM 組件，重點關注計算性能優化。

## 架構設計

### Crates 結構

```
bom/
├── bom-core/          # 核心數據模型（與 SAP/Oracle 相容）
├── bom-graph/         # 自建圖數據結構（針對 BOM 優化）
├── bom-calc/          # 計算引擎（物料展開、成本、Where-Used）
├── bom-cache/         # 緩存層（moka + redb）[待實現]
├── bom-ffi/           # C FFI 綁定（用於其他語言調用）[待實現]
└── bom-adapters/      # PLM/ERP 適配器（SAP, Oracle）[待實現]
```

## 核心功能

### ✅ 已實現

#### 1. **自建圖結構 (bom-graph)**
- **Arena-based 內存分配**：提升緩存局部性
- **增量計算支持**：dirty flag 追蹤變更
- **並行遍歷**：支持拓撲排序、層級分組
- **循環依賴檢測**：避免無限遞歸

#### 2. **物料展開 (Material Explosion)**
- 多層級 BOM 展開
- 並行計算（使用 rayon + 拓撲分層）
- 損耗率計算
- 路徑追蹤（所有從根到葉的路徑）

#### 3. **成本計算 (Costing)**
- 多層級成本累加
- 批量並行計算
- 成本驅動分析（找出最貴的組件）
- 支持緩存與增量更新

#### 4. **Where-Used 分析**
- 反查零件用在哪些產品
- 變更影響分析（ECO Impact Analysis）
- 共用件識別

#### 5. **SAP/Oracle 相容數據模型**
- 生效日期範圍 (Effectivity)
- 替代料組 (Alternative Groups)
- 幻影件 (Phantom Components)
- 多組織/工廠支持
- 版本控制（樂觀鎖）

### 🚧 待實現

1. **緩存層** (bom-cache)
   - L1: 內存緩存 (moka)
   - L2: 持久化 (redb)

2. **FFI 綁定** (bom-ffi)
   - C ABI 接口
   - 支持 Java/Python/.NET 調用

3. **性能 Benchmark**
   - 大規模 BOM 測試（1000+ 組件）
   - 並行計算性能測試

4. **PLM/ERP 適配器**
   - SAP BAPI/OData 接口
   - Oracle REST API 接口

## 技術選型

| 功能 | 技術 | 理由 |
|------|------|------|
| 圖結構 | 自建 Arena-based | 針對 BOM 優化，更好的緩存局部性 |
| 並行計算 | rayon | 簡單易用，work-stealing 負載均衡 |
| 緩存 | moka + redb | 內存 + 持久化分層緩存 |
| 序列化 | serde (json/bincode/msgpack) | 多格式支持 |
| FFI | cbindgen | 自動生成 C header |
| 錯誤處理 | thiserror | 類型安全的錯誤定義 |
| 數值計算 | rust_decimal | 精確的財務計算 |

## 快速開始

### 編譯

```bash
cargo build --release
```

### 運行測試

```bash
cargo test
```

### 使用範例

```rust
use bom_core::repository::memory::InMemoryRepository;
use bom_core::*;
use bom_calc::BomEngine;
use rust_decimal::Decimal;

// 建立 Repository
let repo = InMemoryRepository::new();

// 添加組件
repo.add_component(Component {
    id: ComponentId::new("A"),
    // ...
});

// 添加 BOM 結構
repo.add_bom_item(BomItem {
    parent_id: ComponentId::new("A"),
    child_id: ComponentId::new("B"),
    quantity: Decimal::from(2),
    // ...
});

// 建立計算引擎
let engine = BomEngine::new(repo)?;

// 物料展開
let explosion = engine.explode(&ComponentId::new("A"), Decimal::from(10))?;

// 成本計算
let cost = engine.calculate_cost(&ComponentId::new("A"))?;

// Where-Used 分析
let where_used = engine.where_used(&ComponentId::new("B"))?;
```

## 性能特性

1. **並行計算**
   - 同一層級的節點可並行處理
   - 使用 rayon 進行數據並行

2. **增量計算**
   - Dirty flag 標記需要重算的節點
   - 只重算變更的子樹

3. **內存優化**
   - Arena allocator 連續內存分配
   - 減少指針追蹤，提升緩存命中率

4. **零拷貝**
   - 盡可能使用引用傳遞
   - 避免不必要的克隆

## 測試狀態

```bash
$ cargo test
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured
```

✅ 所有測試通過！

## 運行示例

```bash
cd examples/simple
cargo run
```

輸出示例：
```
=== BOM Calculation Example ===

📊 BOM Graph Statistics:
  Components: 4
  Relationships: 4
  Max Depth: 2
  Root Products: 1

🔧 Material Explosion (製造 10 輛自行車):
  Level 0: Bicycle - Quantity: 10
  Level 1: Frame - Quantity: 10
  Level 1: Wheel Set - Quantity: 20
  Level 2: Aluminum Tube - Quantity: 40

💰 Cost Calculation:
  Bicycle Total Cost: $1200
  - Material: $1200

📈 Cost Drivers (Top Contributors):
  1. Frame - $300 (25.0%)
  2. Wheel Set - $200 (16.6%)
  3. Aluminum Tube - $50 (4.1%)

🔍 Where-Used Analysis (鋁管 D 用在哪裡):
  - Frame (qty: 2, level: 2)
  - Wheel Set (qty: 1, level: 2)

⚠️  Change Impact Analysis (如果鋁管 D 變更):
  Affected Components: 3
  Affected Root Products: 1

🔗 Shared Components Analysis:
  Aluminum Tube - used in 2 assemblies

✅ All calculations completed successfully!
```

## 授權

MIT OR Apache-2.0

## 貢獻

歡迎提交 Issue 和 Pull Request！
