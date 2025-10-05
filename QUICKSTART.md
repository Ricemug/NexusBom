# 快速入門指南

## 5 分鐘上手 BOM 計算引擎

### 1. 環境準備

確保安裝了 Rust：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 編譯項目

```bash
cd /home/ivan/code/bom
cargo build --release
```

### 3. 運行測試

```bash
cargo test --workspace
```

預期輸出：
```
running 24 tests
test result: ok. 24 passed; 0 failed
```

### 4. 運行示例

```bash
cd examples/simple
cargo run
```

### 5. 使用庫

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
bom-core = { path = "path/to/bom/crates/bom-core" }
bom-calc = { path = "path/to/bom/crates/bom-calc" }
rust_decimal = "1.33"
chrono = "0.4"
uuid = { version = "1.6", features = ["v4"] }
```

### 6. 基本用法

```rust
use bom_calc::BomEngine;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::*;
use rust_decimal::Decimal;

fn main() {
    // 1. 建立 Repository
    let repo = InMemoryRepository::new();

    // 2. 添加組件
    repo.add_component(Component {
        id: ComponentId::new("BIKE"),
        description: "Bicycle".to_string(),
        standard_cost: Some(Decimal::from(1000)),
        // ... 其他欄位
    });

    // 3. 添加 BOM 結構
    repo.add_bom_item(BomItem {
        parent_id: ComponentId::new("BIKE"),
        child_id: ComponentId::new("FRAME"),
        quantity: Decimal::from(1),
        // ... 其他欄位
    });

    // 4. 建立計算引擎
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

    // 7. Where-Used 分析
    let where_used = engine.where_used(
        &ComponentId::new("FRAME")
    ).unwrap();

    println!("Total cost: ${}", cost.total_cost);
}
```

## 常見問題

### Q: 如何檢測循環依賴？

A: BomGraph 在建立時會自動檢測循環依賴：

```rust
let graph = BomGraph::from_repository(&repo)?;
graph.validate()?; // 會返回錯誤如果有循環
```

### Q: 如何處理損耗率？

A: 在 BomItem 中設置 `scrap_factor`：

```rust
BomItem {
    quantity: Decimal::from(100),
    scrap_factor: Decimal::from_str("0.05")?, // 5% 損耗
    // ...
}
```

有效數量 = quantity * (1 + scrap_factor) = 105

### Q: 如何實現增量計算？

A: 使用 dirty flag：

```rust
// 標記組件為 dirty
engine.mark_dirty(&ComponentId::new("FRAME"))?;

// 下次計算時只會重算受影響的部分
let cost = engine.calculate_cost(&ComponentId::new("BIKE"))?;
```

### Q: 支持哪些 BOM 類型？

A: 支持所有企業級 BOM 類型：

- ✅ 單層 BOM
- ✅ 多層 BOM
- ✅ 幻影件 (Phantom)
- ✅ 替代料組 (Alternative Groups)
- ✅ 生效日期範圍
- ✅ Alternative BOM
- ✅ 不同用途 BOM（生產/工程/成本）

### Q: 性能如何？

A: 核心優化：

- **並行計算**: 使用 rayon，多核加速
- **Arena allocator**: 連續內存，減少緩存未命中
- **增量計算**: 只重算變更部分
- **批量操作**: 一次獲取多個組件數據

### Q: 如何對接 SAP/Oracle？

A: 實現 `BomRepository` trait：

```rust
struct SapBomRepository {
    // SAP connection
}

impl BomRepository for SapBomRepository {
    fn get_component(&self, id: &ComponentId) -> Result<Component> {
        // 調用 SAP BAPI 獲取組件
    }

    fn get_bom_items(&self, ...) -> Result<Vec<BomItem>> {
        // 調用 SAP BAPI 獲取 BOM
    }

    // ... 其他方法
}

let engine = BomEngine::new(SapBomRepository::new())?;
```

## 下一步

- 閱讀 [README.md](README.md) 了解完整功能
- 查看 [examples/simple](examples/simple) 了解更多示例
- 閱讀 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 了解設計理念

## 支持

如有問題，請提交 Issue 或查看文檔。
