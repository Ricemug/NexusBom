# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2025-10-04

### Added

#### Core Features
- **自建圖結構 (bom-graph)**
  - Arena-based 內存分配器，提升緩存局部性
  - 增量計算支持（dirty flag）
  - 循環依賴檢測
  - 拓撲排序和層級分組

- **物料展開 (Material Explosion)**
  - 多層級 BOM 展開
  - 並行計算（使用 rayon）
  - 損耗率計算
  - 路徑追蹤

- **成本計算 (Costing)**
  - 多層級成本累加
  - 批量並行計算
  - 成本驅動分析
  - 支持緩存與增量更新

- **Where-Used 分析**
  - 反查零件用在哪些產品
  - 變更影響分析 (ECO Impact)
  - 共用件識別

- **SAP/Oracle 相容數據模型**
  - 生效日期範圍 (Effectivity)
  - 替代料組 (Alternative Groups)
  - 幻影件 (Phantom Components)
  - 多組織/工廠支持
  - 版本控制（樂觀鎖）

#### Infrastructure
- Cargo workspace 結構（6 個 crates）
- 完整的測試覆蓋（12 個單元測試，全部通過）
- 示例程序 (`examples/simple`)
- 詳細的 README 文檔

### Technical Details

- **並行計算**: 使用 rayon 進行數據並行，支持層級並行處理
- **性能優化**: Arena allocator、零拷貝、增量計算
- **依賴管理**: 使用 workspace 共享依賴版本
- **錯誤處理**: 使用 thiserror 進行類型安全的錯誤定義

### Known Limitations

- 緩存層（moka + redb）尚未實現
- FFI 綁定尚未實現
- 性能 benchmark 尚未建立
- 工程與生產 BOM 切換功能待完善
- 路由 (Routing) 整合待實現

### Dependencies

主要依賴：
- `serde` 1.0 - 序列化
- `rayon` 1.11 - 並行計算
- `chrono` 0.4 - 日期時間
- `rust_decimal` 1.38 - 精確數值計算
- `uuid` 1.6 - 唯一識別碼
- `thiserror` 1.0 - 錯誤處理

## [Unreleased]

### Planned Features

- 緩存層實現（moka + redb）
- C FFI 綁定（支持 Java/Python/.NET 調用）
- 性能 benchmark 套件
- SAP/Oracle 適配器實現
- 工程 BOM vs 製造 BOM 支持
- 路由整合（工序綁定）
- SIMD 優化數值計算
