# 🏭 BOM Berechnungs-Engine

> Hochleistungs-Stücklistenberechnungs-Engine für PLM/ERP-Systeme

[English](../README.md) | [繁體中文](./README.zh-TW.md) | [简体中文](./README.zh-CN.md)

[![Lizenz](https://img.shields.io/badge/license-AGPL--3.0%20%7C%20Commercial-blue.svg)](../LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

## 🚀 Hauptmerkmale

- ⚡ **Ultra-schnell**: Graphkonstruktion in Mikrosekunden, Auflösung in <25μs
- 🔧 **SAP/Oracle-kompatibel**: Entwickelt für Enterprise-ERP-Systeme
- 🌐 **Mehrsprachig**: FFI-Bindings für C/C++/Python/Java-Integration
- 💾 **Intelligenter Cache**: Zweistufiger Cache (Speicher + persistent) für optimale Leistung
- 🔄 **Parallele Verarbeitung**: Nutzt Rayon für Mehrkern-Berechnung
- 📊 **Vollständige BOM-Operationen**:
  - Materialauflösung (mehrstufige Stückliste)
  - Kostenberechnung mit Rollup
  - Verwendungsnachweis-Analyse
  - Änderungsauswirkungsanalyse
  - Handhabung von Phantom-Komponenten
  - Unterstützung alternativer Stücklisten

## 📦 Installation

### Rust

```toml
[dependencies]
bom-core = "0.1"
bom-graph = "0.1"
bom-calc = "0.1"
```

### C/C++

```bash
# Vorkompilierte Binärdateien herunterladen
wget https://github.com/yourname/bom/releases/latest/download/libbom_ffi.so

# Oder aus Quellcode kompilieren
cargo build --release -p bom-ffi
```

### Python (über FFI)

```python
# Demnächst verfügbar
pip install bom-engine
```

## 🎯 Schnellstart

```rust
use bom_core::repository::memory::InMemoryRepository;
use bom_graph::BomGraph;
use bom_calc::explosion::ExplosionCalculator;

// Einfache BOM-Struktur erstellen
let mut repo = InMemoryRepository::new();

// Komponenten hinzufügen
repo.add_component(create_component("BIKE", "Fahrrad", 500));
repo.add_component(create_component("FRAME", "Rahmen", 150));
repo.add_component(create_component("WHEEL", "Rad", 50));

// BOM-Beziehungen definieren
repo.add_bom_item(create_bom_item("BIKE", "FRAME", 1));
repo.add_bom_item(create_bom_item("BIKE", "WHEEL", 2));

// Graph erstellen und berechnen
let graph = BomGraph::from_component(&repo, &ComponentId::new("BIKE"), None)?;
let calculator = ExplosionCalculator::new(&graph);
let result = calculator.explode(&ComponentId::new("BIKE"), Decimal::from(10))?;

println!("Benötigte Gesamtartikel: {}", result.items.len());
```

Siehe [examples/simple](../examples/simple) für ein vollständiges Arbeitsbeispiel.

## 📊 Leistung

Benchmark auf Consumer-Hardware:

| Operation | Zeit | Beschreibung |
|-----------|------|--------------|
| Graph-Konstruktion | ~1.2-2.1 μs | BOM-Graphstruktur erstellen |
| Materialauflösung | ~19-24 μs | Mehrstufige Stücklistenauflösung |
| Kostenberechnung | ~21-28 μs | Kostenaufrollung mit Treibern |
| Verwendungsnachweis | ~192 ns | Umgekehrte Stücklistensuche |

Siehe [BENCHMARK_RESULTS.md](../BENCHMARK_RESULTS.md) für detaillierte Metriken.

## 🏗️ Architektur

```
┌─────────────────────────────────────────────┐
│          Anwendungsschicht                   │
│    (ERP/PLM/Kundenspezifische Anwendungen)  │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│           FFI-Schicht (C API)                │
│       bom-ffi (libbom_ffi.so)               │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│         Berechnungs-Engines                  │
│  ┌──────────┬──────────┬──────────────┐    │
│  │Auflösung │  Kosten  │Verwendungs-  │    │
│  │          │          │  nachweis    │    │
│  └──────────┴──────────┴──────────────┘    │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│        Graph & Cache-Schicht                 │
│  ┌─────────────┬────────────────────┐       │
│  │  BOM-Graph  │  Stufiger Cache    │       │
│  │   (Arena)   │  (Moka + Redb)     │       │
│  └─────────────┴────────────────────┘       │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│        Daten-Repository-Schicht              │
│   (SAP/Oracle/Kundenspezifische DB-Adapter) │
└─────────────────────────────────────────────┘
```

## 🔌 Integration

### SAP-Integration

```rust
// SAP-Adapter verwenden (demnächst verfügbar)
use bom_adapters::sap::SapRepository;

let repo = SapRepository::connect(sap_config)?;
let graph = BomGraph::from_component(&repo, &component_id, None)?;
```

### Oracle-Integration

```rust
// Oracle-Adapter verwenden (demnächst verfügbar)
use bom_adapters::oracle::OracleRepository;

let repo = OracleRepository::connect(oracle_config)?;
let graph = BomGraph::from_component(&repo, &component_id, None)?;
```

### Kundenspezifische Datenbank

Implementieren Sie das `BomRepository`-Trait:

```rust
impl BomRepository for MyCustomRepo {
    fn get_component(&self, id: &ComponentId) -> Result<Component>;
    fn get_bom_items(&self, parent_id: &ComponentId, date: Option<DateTime<Utc>>) -> Result<Vec<BomItem>>;
    // ... weitere Methoden
}
```

## 📚 Dokumentation

### API-Dokumentation

API-Dokumentation lokal generieren:

```bash
# Dokumentation generieren und im Browser öffnen
cargo doc --no-deps --open

# Dokumentation für alle Crates generieren
cargo doc --workspace --no-deps
```

Dokumentation verfügbar unter: `target/doc/bom_core/index.html`

### Weitere Dokumentation

- [Schnellstartanleitung](./QUICKSTART.de.md)
- [Architekturübersicht](./PROJECT_SUMMARY.de.md)
- [Benchmarks](./BENCHMARK_RESULTS.de.md)
- [Beitragsleitfaden](./CONTRIBUTING.de.md)

## 🤝 Mitwirken

Wir freuen uns über Beiträge! Bitte siehe [CONTRIBUTING.md](../CONTRIBUTING.md) für Richtlinien.

## 💝 Dieses Projekt unterstützen

Wenn dieses Projekt Ihrem Unternehmen hilft, unterstützen Sie bitte die Entwicklung:

- ⭐ Repository mit Stern markieren
- 🐛 Fehler melden und Funktionen vorschlagen
- 💰 [Auf Ko-fi sponsern](https://ko-fi.com/ivanh0906)
- 💰 [GitHub Sponsors](https://github.com/sponsors/yourname)
- 🏢 [Kommerzielle Lizenz](../COMMERCIAL-LICENSE.md) für Unternehmenseinsatz

## 📄 Lizenz

Dieses Projekt ist dual-lizenziert:

- **Open Source**: [AGPL-3.0](../LICENSE) für Open-Source-Projekte
- **Kommerziell**: [Kommerzielle Lizenz](../COMMERCIAL-LICENSE.md) für proprietäre Nutzung

Wählen Sie die Lizenz, die Ihren Anforderungen am besten entspricht.

## 🌟 Verwendet von

- Ihr Firmenname hier
- [Reichen Sie einen PR ein, um Ihren hinzuzufügen!]

## 📞 Kontakt

- **Probleme**: [GitHub Issues](https://github.com/yourname/bom/issues)
- **Diskussionen**: [GitHub Discussions](https://github.com/yourname/bom/discussions)
- **E-Mail**: xiaoivan1@proton.me

---

Mit ❤️ für die Fertigungsindustrie entwickelt
