# 🏭 BOM 计算引擎

> 高性能物料清单计算引擎，专为 PLM/ERP 系统设计

[English](../README.md) | [繁體中文](./README.zh-TW.md) | [Deutsch](./README.de.md)

[![许可证](https://img.shields.io/badge/license-AGPL--3.0%20%7C%20Commercial-blue.svg)](../LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![测试](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

## 🚀 核心特性

- ⚡ **超高性能**：图结构构建仅需微秒，物料展开 <25μs
- 🔧 **SAP/Oracle 兼容**：为企业级 ERP 系统量身打造
- 🌐 **多语言支持**：提供 C/C++/Python/Java 的 FFI 绑定
- 💾 **智能缓存**：双层缓存（内存 + 持久化）实现最优性能
- 🔄 **并行处理**：利用 Rayon 发挥多核计算能力
- 📊 **完整 BOM 功能**：
  - 物料展开（多级 BOM）
  - 成本计算与汇总
  - 反查分析（Where-Used）
  - 变更影响分析
  - 虚拟件处理
  - 替代 BOM 支持

## 📦 安装方式

### Rust

```toml
[dependencies]
bom-core = "0.1"
bom-graph = "0.1"
bom-calc = "0.1"
```

### C/C++

```bash
# 下载预编译二进制文件
wget https://github.com/yourname/bom/releases/latest/download/libbom_ffi.so

# 或从源码编译
cargo build --release -p bom-ffi
```

### Python（通过 FFI）

```python
# 即将推出
pip install bom-engine
```

## 🎯 快速入门

```rust
use bom_core::repository::memory::InMemoryRepository;
use bom_graph::BomGraph;
use bom_calc::explosion::ExplosionCalculator;

// 创建简单的 BOM 结构
let mut repo = InMemoryRepository::new();

// 添加组件
repo.add_component(create_component("BIKE", "自行车", 500));
repo.add_component(create_component("FRAME", "车架", 150));
repo.add_component(create_component("WHEEL", "车轮", 50));

// 定义 BOM 关系
repo.add_bom_item(create_bom_item("BIKE", "FRAME", 1));
repo.add_bom_item(create_bom_item("BIKE", "WHEEL", 2));

// 构建图并计算
let graph = BomGraph::from_component(&repo, &ComponentId::new("BIKE"), None)?;
let calculator = ExplosionCalculator::new(&graph);
let result = calculator.explode(&ComponentId::new("BIKE"), Decimal::from(10))?;

println!("需要的总项目数：{}", result.items.len());
```

完整示例请参考 [examples/simple](../examples/simple)。

## 📊 性能表现

在普通消费级硬件上的测试结果：

| 操作 | 时间 | 说明 |
|------|------|------|
| 图结构构建 | ~1.2-2.1 μs | 构建 BOM 图结构 |
| 物料展开 | ~19-24 μs | 多级 BOM 展开 |
| 成本计算 | ~21-28 μs | 成本汇总与分析 |
| 反查查询 | ~192 ns | 逆向 BOM 查找 |

详细指标请参阅 [BENCHMARK_RESULTS.md](../BENCHMARK_RESULTS.md)。

## 🏗️ 架构设计

```
┌─────────────────────────────────────────────┐
│            应用层                            │
│    (ERP/PLM/定制应用程序)                   │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│           FFI 层 (C API)                     │
│       bom-ffi (libbom_ffi.so)               │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│            计算引擎                          │
│  ┌──────────┬──────────┬──────────────┐    │
│  │物料展开  │  成本    │  反查分析    │    │
│  └──────────┴──────────┴──────────────┘    │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│         图结构 & 缓存层                      │
│  ┌─────────────┬────────────────────┐       │
│  │  BOM 图     │  分层缓存          │       │
│  │  (Arena)    │  (Moka + Redb)     │       │
│  └─────────────┴────────────────────┘       │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│          数据访问层                          │
│     (SAP/Oracle/定制数据库适配器)           │
└─────────────────────────────────────────────┘
```

## 🔌 系统集成

### SAP 集成

```rust
// 使用 SAP 适配器（即将推出）
use bom_adapters::sap::SapRepository;

let repo = SapRepository::connect(sap_config)?;
let graph = BomGraph::from_component(&repo, &component_id, None)?;
```

### Oracle 集成

```rust
// 使用 Oracle 适配器（即将推出）
use bom_adapters::oracle::OracleRepository;

let repo = OracleRepository::connect(oracle_config)?;
let graph = BomGraph::from_component(&repo, &component_id, None)?;
```

### 自定义数据库

实现 `BomRepository` trait：

```rust
impl BomRepository for MyCustomRepo {
    fn get_component(&self, id: &ComponentId) -> Result<Component>;
    fn get_bom_items(&self, parent_id: &ComponentId, date: Option<DateTime<Utc>>) -> Result<Vec<BomItem>>;
    // ... 其他方法
}
```

## 📚 技术文档

### API 文档

在本地生成 API 文档：

```bash
# 生成并在浏览器中打开文档
cargo doc --no-deps --open

# 生成所有 crate 的文档
cargo doc --workspace --no-deps
```

文档位置：`target/doc/bom_core/index.html`

### 其他文档

- [快速入门](./QUICKSTART.zh-CN.md)
- [架构概述](./PROJECT_SUMMARY.zh-CN.md)
- [性能测试](./BENCHMARK_RESULTS.zh-CN.md)
- [贡献指南](./CONTRIBUTING.zh-CN.md)

## 🤝 贡献参与

我们欢迎各种形式的贡献！请参阅 [CONTRIBUTING.md](../CONTRIBUTING.md) 了解贡献准则。

## 💝 支持本项目

如果这个项目对您的业务有帮助，请考虑支持开发：

- ⭐ 给项目点个星
- 🐛 报告问题并提出功能建议
- 💰 [Ko-fi 赞助](https://ko-fi.com/ivanh0906)
- 💰 [GitHub Sponsors](https://github.com/sponsors/yourname)
- 🏢 [商业许可](../COMMERCIAL-LICENSE.md) 供企业使用

## 📄 许可方式

本项目采用双重许可：

- **开源许可**：[AGPL-3.0](../LICENSE) 供开源项目使用
- **商业许可**：[商业许可](../COMMERCIAL-LICENSE.md) 供专有软件使用

请选择最适合您需求的许可方式。

## 🌟 使用案例

- 您的公司名称
- [提交 PR 加入您的案例！]

## 📞 联系方式

- **问题报告**：[GitHub Issues](https://github.com/yourname/bom/issues)
- **讨论交流**：[GitHub Discussions](https://github.com/yourname/bom/discussions)
- **电子邮箱**：xiaoivan1@proton.me

---

为制造业用 ❤️ 打造
