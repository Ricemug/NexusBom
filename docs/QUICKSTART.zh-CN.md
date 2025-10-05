# 快速入门指南

[English](../QUICKSTART.md) | [繁體中文](./QUICKSTART.zh-TW.md) | [Deutsch](./QUICKSTART.de.md)

## 5 分钟快速上手 BOM Engine

### 1. 环境设置

安装 Rust：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 构建项目

```bash
git clone https://github.com/Ricemug/bom
cd bom
cargo build --release
```

### 3. 运行测试

```bash
cargo test --workspace
```

预期输出：
```
running 24 tests
test result: ok. 24 passed; 0 failed
```

### 4. 运行示例

```bash
cd examples/simple
cargo run
```

### 5. 作为库使用

将以下内容添加到您的 `Cargo.toml`：

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
    // 1. 创建仓库
    let repo = InMemoryRepository::new();

    // 2. 添加组件
    repo.add_component(Component {
        id: ComponentId::new("BIKE"),
        description: "Bicycle".to_string(),
        standard_cost: Some(Decimal::from(1000)),
        // ... 其他字段
    });

    // 3. 添加 BOM 结构
    repo.add_bom_item(BomItem {
        parent_id: ComponentId::new("BIKE"),
        child_id: ComponentId::new("FRAME"),
        quantity: Decimal::from(1),
        // ... 其他字段
    });

    // 4. 创建 BOM Engine
    let engine = BomEngine::new(repo).unwrap();

    // 5. 物料展开
    let explosion = engine.explode(
        &ComponentId::new("BIKE"),
        Decimal::from(10)
    ).unwrap();

    // 6. 成本计算
    let cost = engine.calculate_cost(
        &ComponentId::new("BIKE")
    ).unwrap();

    // 7. 反查分析
    let where_used = engine.where_used(
        &ComponentId::new("FRAME")
    ).unwrap();

    println!("总成本: ${}", cost.total_cost);
}
```

## 常见问题

### 问：如何检测循环依赖？

答：BomGraph 会在构建时自动检测循环依赖：

```rust
let graph = BomGraph::from_repository(&repo)?;
graph.validate()?; // 如果存在循环会返回错误
```

### 问：如何处理损耗率？

答：在 BomItem 中设置 `scrap_factor`：

```rust
BomItem {
    quantity: Decimal::from(100),
    scrap_factor: Decimal::from_str("0.05")?, // 5% 损耗
    // ...
}
```

实际数量 = quantity * (1 + scrap_factor) = 105

### 问：如何实现增量计算？

答：使用脏标记（dirty flags）：

```rust
// 标记组件为脏
engine.mark_dirty(&ComponentId::new("FRAME"))?;

// 下次计算只会重新计算受影响的部分
let cost = engine.calculate_cost(&ComponentId::new("BIKE"))?;
```

### 问：支持哪些 BOM 类型？

答：所有企业级 BOM 类型：

- ✅ 单层 BOM
- ✅ 多层 BOM
- ✅ 虚拟件
- ✅ 替代组
- ✅ 日期有效性
- ✅ 替代 BOM
- ✅ 不同 BOM 用途（生产/工程/成本）

### 问：性能如何？

答：核心优化：

- **并行计算**：使用 rayon 进行多核加速
- **Arena 分配器**：连续内存以获得更好的缓存局部性
- **增量计算**：只重新计算变更的部分
- **批量操作**：一次获取多个组件

### 问：如何集成 SAP/Oracle？

答：实现 `BomRepository` trait：

```rust
struct SapBomRepository {
    // SAP 连接
}

impl BomRepository for SapBomRepository {
    fn get_component(&self, id: &ComponentId) -> Result<Component> {
        // 调用 SAP BAPI 获取组件
    }

    fn get_bom_items(&self, ...) -> Result<Vec<BomItem>> {
        // 调用 SAP BAPI 获取 BOM
    }

    // ... 其他方法
}

let engine = BomEngine::new(SapBomRepository::new())?;
```

## 下一步

- 阅读 [README.md](README.md) 了解完整功能
- 查看 [examples/simple](examples/simple) 获得更多示例
- 阅读 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 了解设计理念

## 支持

如有问题，请提交 issue 或查看文档。
