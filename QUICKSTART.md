# Quick Start Guide

[繁體中文](./docs/QUICKSTART.zh-TW.md) | [简体中文](./docs/QUICKSTART.zh-CN.md) | [Deutsch](./docs/QUICKSTART.de.md)

## Get Started with BOM Engine in 5 Minutes

### 1. Environment Setup

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build the Project

```bash
git clone https://github.com/Ricemug/bom
cd bom
cargo build --release
```

### 3. Run Tests

```bash
cargo test --workspace
```

Expected output:
```
running 24 tests
test result: ok. 24 passed; 0 failed
```

### 4. Run Examples

```bash
cd examples/simple
cargo run
```

### 5. Use as Library

Add to your `Cargo.toml`:

```toml
[dependencies]
bom-core = { git = "https://github.com/Ricemug/bom" }
bom-calc = { git = "https://github.com/Ricemug/bom" }
bom-graph = { git = "https://github.com/Ricemug/bom" }
rust_decimal = "1.33"
chrono = "0.4"
uuid = { version = "1.6", features = ["v4"] }
```

### 6. Basic Usage

```rust
use bom_calc::BomEngine;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::*;
use rust_decimal::Decimal;

fn main() {
    // 1. Create Repository
    let repo = InMemoryRepository::new();

    // 2. Add Components
    repo.add_component(Component {
        id: ComponentId::new("BIKE"),
        description: "Bicycle".to_string(),
        standard_cost: Some(Decimal::from(1000)),
        // ... other fields
    });

    // 3. Add BOM Structure
    repo.add_bom_item(BomItem {
        parent_id: ComponentId::new("BIKE"),
        child_id: ComponentId::new("FRAME"),
        quantity: Decimal::from(1),
        // ... other fields
    });

    // 4. Create BOM Engine
    let engine = BomEngine::new(repo).unwrap();

    // 5. Material Explosion
    let explosion = engine.explode(
        &ComponentId::new("BIKE"),
        Decimal::from(10)
    ).unwrap();

    // 6. Cost Calculation
    let cost = engine.calculate_cost(
        &ComponentId::new("BIKE")
    ).unwrap();

    // 7. Where-Used Analysis
    let where_used = engine.where_used(
        &ComponentId::new("FRAME")
    ).unwrap();

    println!("Total cost: ${}", cost.total_cost);
}
```

## FAQ

### Q: How to detect circular dependencies?

A: BomGraph automatically detects circular dependencies during construction:

```rust
let graph = BomGraph::from_repository(&repo)?;
graph.validate()?; // Returns error if there are cycles
```

### Q: How to handle scrap factors?

A: Set `scrap_factor` in BomItem:

```rust
BomItem {
    quantity: Decimal::from(100),
    scrap_factor: Decimal::from_str("0.05")?, // 5% scrap
    // ...
}
```

Effective quantity = quantity * (1 + scrap_factor) = 105

### Q: How to implement incremental calculation?

A: Use dirty flags:

```rust
// Mark component as dirty
engine.mark_dirty(&ComponentId::new("FRAME"))?;

// Next calculation only recomputes affected parts
let cost = engine.calculate_cost(&ComponentId::new("BIKE"))?;
```

### Q: Which BOM types are supported?

A: All enterprise-grade BOM types:

- ✅ Single-level BOM
- ✅ Multi-level BOM
- ✅ Phantom Parts
- ✅ Alternative Groups
- ✅ Date Effectivity
- ✅ Alternate BOMs
- ✅ Different BOM Usages (Production/Engineering/Costing)

### Q: How is the performance?

A: Core optimizations:

- **Parallel Computation**: Using rayon for multi-core acceleration
- **Arena Allocator**: Contiguous memory for better cache locality
- **Incremental Calculation**: Only recompute changed parts
- **Batch Operations**: Fetch multiple components at once

### Q: How to integrate with SAP/Oracle?

A: Implement the `BomRepository` trait:

```rust
struct SapBomRepository {
    // SAP connection
}

impl BomRepository for SapBomRepository {
    fn get_component(&self, id: &ComponentId) -> Result<Component> {
        // Call SAP BAPI to get component
    }

    fn get_bom_items(&self, ...) -> Result<Vec<BomItem>> {
        // Call SAP BAPI to get BOM
    }

    // ... other methods
}

let engine = BomEngine::new(SapBomRepository::new())?;
```

## Next Steps

- Read [README.md](README.md) for full features
- Check [examples/simple](examples/simple) for more examples
- Read [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) for design philosophy

## Support

For questions, please submit an issue or check the documentation.
