# 快速入門指南

[English](../QUICKSTART.md) | [简体中文](./QUICKSTART.zh-CN.md) | [Deutsch](./QUICKSTART.de.md)

## 5 分鐘快速上手 BOM Engine

### 1. 環境設定

安裝 Rust：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 建置專案

```bash
git clone https://github.com/Ricemug/bom
cd bom
cargo build --release
```

### 3. 執行測試

```bash
cargo test --workspace
```

預期輸出：
```
running 24 tests
test result: ok. 24 passed; 0 failed
```

### 4. 執行範例

```bash
cd examples/simple
cargo run
```

### 5. 作為函式庫使用

將以下內容加入您的 `Cargo.toml`：

```toml
[dependencies]
bom-core = { git = "https://github.com/Ricemug/bom" }
bom-calc = { git = "https://github.com/Ricemug/bom" }
bom-graph = { git = "https://github.com/Ricemug/bom" }
rust_decimal = "1.33"
chrono = "0.4"
uuid = { version = "1.6", features = ["v4"] }
```

### 6. 基本使用

```rust
use bom_calc::BomEngine;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::*;
use rust_decimal::Decimal;

fn main() {
    // 1. 建立儲存庫
    let repo = InMemoryRepository::new();

    // 2. 新增組件
    repo.add_component(Component {
        id: ComponentId::new("BIKE"),
        description: "Bicycle".to_string(),
        standard_cost: Some(Decimal::from(1000)),
        // ... 其他欄位
    });

    // 3. 新增 BOM 結構
    repo.add_bom_item(BomItem {
        parent_id: ComponentId::new("BIKE"),
        child_id: ComponentId::new("FRAME"),
        quantity: Decimal::from(1),
        // ... 其他欄位
    });

    // 4. 建立 BOM Engine
    let engine = BomEngine::new(repo).unwrap();

    // 5. 物料展開
    let explosion = engine.explode(
        &ComponentId::new("BIKE"),
        Decimal::from(10)
    ).unwrap();

    // 6. 成本計算
    let cost = engine.calculate_cost(
        &ComponentId::new("BIKE")
    ).unwrap();

    // 7. 反查分析
    let where_used = engine.where_used(
        &ComponentId::new("FRAME")
    ).unwrap();

    println!("總成本: ${}", cost.total_cost);
}
```

## 常見問題

### 問：如何偵測循環相依？

答：BomGraph 會在建構時自動偵測循環相依：

```rust
let graph = BomGraph::from_repository(&repo)?;
graph.validate()?; // 如果存在循環會回傳錯誤
```

### 問：如何處理損耗率？

答：在 BomItem 中設定 `scrap_factor`：

```rust
BomItem {
    quantity: Decimal::from(100),
    scrap_factor: Decimal::from_str("0.05")?, // 5% 損耗
    // ...
}
```

實際數量 = quantity * (1 + scrap_factor) = 105

### 問：如何實現增量計算？

答：使用髒標記（dirty flags）：

```rust
// 標記組件為髒
engine.mark_dirty(&ComponentId::new("FRAME"))?;

// 下次計算只會重新計算受影響的部分
let cost = engine.calculate_cost(&ComponentId::new("BIKE"))?;
```

### 問：支援哪些 BOM 類型？

答：所有企業級 BOM 類型：

- ✅ 單階 BOM
- ✅ 多階 BOM
- ✅ 虛擬件
- ✅ 替代群組
- ✅ 日期有效性
- ✅ 替代 BOM
- ✅ 不同 BOM 用途（生產/工程/成本）

### 問：效能如何？

答：核心優化：

- **平行計算**：使用 rayon 進行多核心加速
- **Arena 配置器**：連續記憶體以獲得更好的快取局部性
- **增量計算**：只重新計算變更的部分
- **批次操作**：一次取得多個組件

### 問：如何整合 SAP/Oracle？

答：實作 `BomRepository` trait：

```rust
struct SapBomRepository {
    // SAP 連接
}

impl BomRepository for SapBomRepository {
    fn get_component(&self, id: &ComponentId) -> Result<Component> {
        // 呼叫 SAP BAPI 取得組件
    }

    fn get_bom_items(&self, ...) -> Result<Vec<BomItem>> {
        // 呼叫 SAP BAPI 取得 BOM
    }

    // ... 其他方法
}

let engine = BomEngine::new(SapBomRepository::new())?;
```

## 下一步

- 閱讀 [README.md](README.md) 了解完整功能
- 查看 [examples/simple](examples/simple) 獲得更多範例
- 閱讀 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 了解設計理念

## 支援

如有問題，請提交 issue 或查看文件。
