# Schnellstart-Anleitung

[English](../QUICKSTART.md) | [繁體中文](./QUICKSTART.zh-TW.md) | [简体中文](./QUICKSTART.zh-CN.md)

## BOM Engine in 5 Minuten starten

### 1. Umgebungseinrichtung

Rust installieren:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Projekt bauen

```bash
git clone https://github.com/Ricemug/NexusBom
cd bom
cargo build --release
```

### 3. Tests ausführen

```bash
cargo test --workspace
```

Erwartete Ausgabe:
```
running 24 tests
test result: ok. 24 passed; 0 failed
```

### 4. Beispiele ausführen

```bash
cd examples/simple
cargo run
```

### 5. Als Bibliothek verwenden

Fügen Sie zu Ihrer `Cargo.toml` hinzu:

```toml
[dependencies]
bom-core = { git = "https://github.com/Ricemug/NexusBom" }
bom-calc = { git = "https://github.com/Ricemug/NexusBom" }
bom-graph = { git = "https://github.com/Ricemug/NexusBom" }
rust_decimal = "1.33"
chrono = "0.4"
uuid = { version = "1.6", features = ["v4"] }
```

### 6. Grundlegende Verwendung

```rust
use bom_calc::BomEngine;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::*;
use rust_decimal::Decimal;

fn main() {
    // 1. Repository erstellen
    let repo = InMemoryRepository::new();

    // 2. Komponenten hinzufügen
    repo.add_component(Component {
        id: ComponentId::new("BIKE"),
        description: "Bicycle".to_string(),
        standard_cost: Some(Decimal::from(1000)),
        // ... weitere Felder
    });

    // 3. BOM-Struktur hinzufügen
    repo.add_bom_item(BomItem {
        parent_id: ComponentId::new("BIKE"),
        child_id: ComponentId::new("FRAME"),
        quantity: Decimal::from(1),
        // ... weitere Felder
    });

    // 4. BOM Engine erstellen
    let engine = BomEngine::new(repo).unwrap();

    // 5. Material-Aufschlüsselung
    let explosion = engine.explode(
        &ComponentId::new("BIKE"),
        Decimal::from(10)
    ).unwrap();

    // 6. Kostenberechnung
    let cost = engine.calculate_cost(
        &ComponentId::new("BIKE")
    ).unwrap();

    // 7. Where-Used-Analyse
    let where_used = engine.where_used(
        &ComponentId::new("FRAME")
    ).unwrap();

    println!("Gesamtkosten: ${}", cost.total_cost);
}
```

## Häufig gestellte Fragen

### F: Wie können zirkuläre Abhängigkeiten erkannt werden?

A: BomGraph erkennt automatisch zirkuläre Abhängigkeiten während der Konstruktion:

```rust
let graph = BomGraph::from_repository(&repo)?;
graph.validate()?; // Gibt einen Fehler zurück, wenn Zyklen vorhanden sind
```

### F: Wie können Ausschussfaktoren behandelt werden?

A: Setzen Sie `scrap_factor` in BomItem:

```rust
BomItem {
    quantity: Decimal::from(100),
    scrap_factor: Decimal::from_str("0.05")?, // 5% Ausschuss
    // ...
}
```

Effektive Menge = quantity * (1 + scrap_factor) = 105

### F: Wie kann eine inkrementelle Berechnung implementiert werden?

A: Verwenden Sie Dirty-Flags:

```rust
// Komponente als dirty markieren
engine.mark_dirty(&ComponentId::new("FRAME"))?;

// Nächste Berechnung berechnet nur betroffene Teile neu
let cost = engine.calculate_cost(&ComponentId::new("BIKE"))?;
```

### F: Welche BOM-Typen werden unterstützt?

A: Alle BOM-Typen auf Enterprise-Niveau:

- ✅ Einstufige BOM
- ✅ Mehrstufige BOM
- ✅ Phantom-Teile
- ✅ Alternativgruppen
- ✅ Datumswirksamkeit
- ✅ Alternative BOMs
- ✅ Verschiedene BOM-Verwendungen (Produktion/Engineering/Kalkulation)

### F: Wie ist die Performance?

A: Kern-Optimierungen:

- **Parallele Berechnung**: Verwendung von rayon für Multi-Core-Beschleunigung
- **Arena-Allocator**: Zusammenhängender Speicher für bessere Cache-Lokalität
- **Inkrementelle Berechnung**: Nur geänderte Teile werden neu berechnet
- **Batch-Operationen**: Mehrere Komponenten auf einmal abrufen

### F: Wie kann SAP/Oracle integriert werden?

A: Implementieren Sie das `BomRepository`-Trait:

```rust
struct SapBomRepository {
    // SAP-Verbindung
}

impl BomRepository for SapBomRepository {
    fn get_component(&self, id: &ComponentId) -> Result<Component> {
        // SAP BAPI aufrufen, um Komponente zu erhalten
    }

    fn get_bom_items(&self, ...) -> Result<Vec<BomItem>> {
        // SAP BAPI aufrufen, um BOM zu erhalten
    }

    // ... weitere Methoden
}

let engine = BomEngine::new(SapBomRepository::new())?;
```

## Nächste Schritte

- Lesen Sie [README.md](README.md) für vollständige Funktionen
- Überprüfen Sie [examples/simple](examples/simple) für weitere Beispiele
- Lesen Sie [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) für die Design-Philosophie

## Support

Bei Fragen reichen Sie bitte ein Issue ein oder konsultieren Sie die Dokumentation.
