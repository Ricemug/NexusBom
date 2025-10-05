# 🏭 BOM Calculation Engine

> High-performance Bill of Materials calculation engine designed for PLM/ERP systems

[繁體中文](./docs/README.zh-TW.md) | [简体中文](./docs/README.zh-CN.md) | [Deutsch](./docs/README.de.md)

[![License](https://img.shields.io/badge/license-AGPL--3.0%20%7C%20Commercial-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

## 🚀 Features

- ⚡ **Blazing Fast**: Graph construction in microseconds, material explosion <25μs
- 🔧 **SAP/Oracle Compatible**: Designed for enterprise-grade ERP systems
- 🌐 **Multi-language Support**: FFI bindings for C/C++/Python/Java
- 💾 **Smart Caching**: Two-tier caching (memory + persistent) for optimal performance
- 🔄 **Parallel Processing**: Leverages Rayon for multi-core computation
- 📊 **Complete BOM Functionality**:
  - Material Explosion (multi-level BOM)
  - Cost Calculation & Roll-up
  - Where-Used Analysis
  - Engineering Change Impact Analysis
  - Phantom Parts Handling
  - Alternate BOM Support

## 📦 Installation

### Rust

```toml
[dependencies]
bom-core = { git = "https://github.com/Ricemug/bom" }
bom-calc = { git = "https://github.com/Ricemug/bom" }
bom-graph = { git = "https://github.com/Ricemug/bom" }
```

### C/C++ (via FFI)

```bash
git clone https://github.com/Ricemug/bom
cd bom/crates/bom-ffi
cargo build --release
# Header: target/release/include/bom.h
# Library: target/release/libbom_ffi.so
```

## 🎯 Quick Start

### Basic Example

```rust
use bom_core::*;
use bom_calc::*;
use rust_decimal::Decimal;

// Create components
let laptop = Component {
    id: ComponentId::new("LAPTOP-001"),
    description: "Laptop Computer".to_string(),
    component_type: ComponentType::FinishedProduct,
    standard_cost: Some(Decimal::new(50000, 2)), // $500
    // ... other fields
};

// Create BOM structure
let bom_items = vec![
    BomItem {
        parent_id: ComponentId::new("LAPTOP-001"),
        child_id: ComponentId::new("CPU-001"),
        quantity: Decimal::ONE,
        // ... other fields
    },
    // ... more items
];

// Build BOM graph
let graph = BomGraph::from_components(&components, &bom_items)?;

// Material explosion
let explosion = ExplosionCalculator::new(&graph).explode(&laptop.id, Decimal::ONE)?;

// Cost calculation
let cost = CostCalculator::new(&graph).calculate(&laptop.id)?;

println!("Total cost: ${}", cost.total_cost);
```

## 📊 Performance Benchmarks

Tested on AMD Ryzen 9 7950X, single-threaded:

| Operation | Time | Throughput |
|-----------|------|------------|
| Graph Build | ~1.2 μs | 833K ops/sec |
| Material Explosion | ~19 μs | 52K ops/sec |
| Cost Calculation | ~21 μs | 47K ops/sec |
| Where-Used Query | ~192 ns | 5.2M ops/sec |

*See [BENCHMARK_RESULTS.md](./BENCHMARK_RESULTS.md) for detailed metrics*

## 🏗️ Architecture

### Crate Structure

```
bom/
├── bom-core/          # Core data models (SAP/Oracle compatible)
├── bom-graph/         # Custom graph structure (arena-based)
├── bom-calc/          # Calculation engines (explosion, costing, where-used)
├── bom-cache/         # Caching layer (moka + redb)
├── bom-ffi/           # C FFI bindings
└── bom-adapters/      # PLM/ERP adapters (SAP, Oracle)
```

### Key Design Decisions

- **Arena-based Memory**: Optimized cache locality for graph traversal
- **Parallel Computation**: Topological sorting + level-wise parallelism
- **Dirty Flag Tracking**: Incremental computation for large BOMs
- **Two-tier Caching**: L1 (memory) + L2 (persistent) for hybrid workloads

## 🔧 Use Cases

- **PLM Systems**: Manage complex product structures
- **ERP Integration**: Real-time BOM explosion for MRP
- **Cost Analysis**: Multi-level cost roll-up and variance analysis
- **Change Management**: Impact analysis for engineering changes
- **Supply Chain**: Component dependency tracking

## 📖 Documentation

- [Quick Start Guide](./QUICKSTART.md)
- [API Documentation](https://docs.rs/bom-core)
- [Architecture Overview](./PROJECT_SUMMARY.md)
- [Benchmarks](./BENCHMARK_RESULTS.md)
- [Contributing Guide](./CONTRIBUTING.md)

## 💼 Licensing

This project is dual-licensed:

- **Open Source**: [AGPL-3.0](./LICENSE) for open-source projects
- **Commercial**: [Commercial License](./COMMERCIAL-LICENSE.md) for proprietary use

For commercial licensing inquiries, contact: xiaoivan1@proton.me

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

Contributions can be made in:
- English
- 繁體中文 (Traditional Chinese)
- 简体中文 (Simplified Chinese)
- Deutsch (German)

## 💖 Support This Project

If you find this project useful, consider supporting development:

[![Ko-fi](https://img.shields.io/badge/Ko--fi-Support-ff5e5b?logo=ko-fi)](https://ko-fi.com/ivanh0906)

## 📜 License

Copyright (c) 2025 BOM Calculation Engine Contributors

Licensed under either:
- AGPL-3.0 License ([LICENSE](./LICENSE))
- Commercial License ([COMMERCIAL-LICENSE.md](./COMMERCIAL-LICENSE.md))

at your option.

## 🌟 Acknowledgments

Built with:
- Rust
- Rayon (parallel processing)
- Arena allocators
- Moka & Redb (caching)

---

**Made with ❤️ for the manufacturing industry**
