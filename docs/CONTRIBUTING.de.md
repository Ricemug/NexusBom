# Beitrag zur BOM-Berechnungs-Engine

[English](../CONTRIBUTING.md) | [繁體中文](./CONTRIBUTING.zh-TW.md) | [简体中文](./CONTRIBUTING.zh-CN.md)

Vielen Dank für Ihr Interesse, einen Beitrag zu leisten! Wir begrüßen Beiträge aus der Community.

## 🌍 Sprachen

Dieses Projekt unterstützt mehrere Sprachen:
- English (primäre Entwicklungssprache)
- 繁體中文 (Traditionelles Chinesisch)
- 简体中文 (Vereinfachtes Chinesisch)
- Deutsch (German)

Dokumentation und Diskussionen können in jeder dieser Sprachen erfolgen.

## 🚀 Erste Schritte

1. Forken Sie das Repository
2. Klonen Sie Ihren Fork: `git clone https://github.com/yourname/bom.git`
3. Erstellen Sie einen Branch: `git checkout -b feature/your-feature-name`
4. Nehmen Sie Ihre Änderungen vor
5. Führen Sie Tests aus: `cargo test`
6. Committen Sie Ihre Änderungen: `git commit -am 'Add some feature'`
7. Pushen Sie zum Branch: `git push origin feature/your-feature-name`
8. Erstellen Sie einen Pull Request

## 📝 Entwicklungsrichtlinien

### Code-Stil

- Folgen Sie dem offiziellen Rust-Stilguide
- Führen Sie `cargo fmt` vor dem Committen aus
- Führen Sie `cargo clippy` aus und beheben Sie Warnungen
- Schreiben Sie Dokumentation für öffentliche APIs
- Fügen Sie Tests für neue Features hinzu

### Commit-Nachrichten

Verwenden Sie klare, beschreibende Commit-Nachrichten:

```
feat: add SAP adapter for BOM repository
fix: correct cost calculation for phantom components
docs: update README with new examples
test: add benchmarks for deep BOM structures
```

### Testen

- Schreiben Sie Unit-Tests für neue Funktionen
- Fügen Sie Integrationstests für neue Features hinzu
- Stellen Sie sicher, dass alle Tests bestehen: `cargo test --all`
- Führen Sie Benchmarks aus, um die Leistung zu überprüfen: `cargo bench`

### Dokumentation

- Aktualisieren Sie die README beim Hinzufügen neuer Features
- Fügen Sie Doc-Kommentare für öffentliche APIs hinzu
- Aktualisieren Sie CHANGELOG.md
- Übersetzen Sie wichtige Dokumentation in unterstützte Sprachen

## 🐛 Fehler melden

Bitte fügen Sie bei der Fehlermeldung Folgendes hinzu:

1. Ihr Betriebssystem und Ihre Rust-Version
2. Schritte zur Reproduktion des Fehlers
3. Erwartetes Verhalten
4. Tatsächliches Verhalten
5. Fehlermeldungen oder Logs

## 💡 Features vorschlagen

Wir lieben neue Ideen! Beim Vorschlagen von Features:

1. Prüfen Sie, ob es bereits vorgeschlagen wurde
2. Erklären Sie den Anwendungsfall
3. Beschreiben Sie die vorgeschlagene Lösung
4. Berücksichtigen Sie die Abwärtskompatibilität

## 🔍 Code-Review-Prozess

1. Alle Einreichungen erfordern eine Überprüfung
2. Wir werden Ihren PR innerhalb weniger Tage überprüfen
3. Bearbeiten Sie jedes Feedback
4. Nach der Genehmigung werden wir Ihren PR zusammenführen

## 📜 Lizenz

Durch Ihren Beitrag erklären Sie sich damit einverstanden, dass Ihre Beiträge unter folgenden Lizenzen lizenziert werden:
- AGPL-3.0 (für Open-Source-Nutzung)
- Commercial License (wie von den Projektbetreuern festgelegt)

## 🙏 Anerkennung

Mitwirkende werden anerkannt in:
- CONTRIBUTORS.md
- Versionshinweisen
- Projektdokumentation

## 💬 Kommunikation

- **GitHub Issues**: Fehlerberichte und Feature-Anfragen
- **GitHub Discussions**: Allgemeine Fragen und Diskussionen
- **Email**: xiaoivan1@proton.me für private Anfragen

## 🎯 Prioritätsbereiche

Wir begrüßen besonders Beiträge in:

1. **ERP/PLM-Adapter**
   - SAP-Connector
   - Oracle-Connector
   - Andere ERP-Systeme

2. **Leistungsverbesserungen**
   - Algorithmusoptimierungen
   - Cache-Verbesserungen
   - Verbesserungen der Parallelverarbeitung

3. **Sprachbindungen**
   - Python-Bindungen
   - Java-Bindungen
   - .NET-Bindungen

4. **Dokumentation**
   - Verwendungsbeispiele
   - Integrationsanleitungen
   - Übersetzungen

5. **Testen**
   - Mehr Testfälle
   - Leistungs-Benchmarks
   - Reale Szenarien

## 📊 Entwicklungsumgebung einrichten

### Voraussetzungen

```bash
# Rust installieren
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Entwicklungstools installieren
cargo install cargo-watch
cargo install cargo-edit
```

### Erstellen

```bash
# Alle Crates erstellen
cargo build --all

# Mit Optimierungen erstellen
cargo build --release

# FFI-Bibliothek erstellen
cargo build --release -p bom-ffi
```

### Testen

```bash
# Alle Tests ausführen
cargo test --all

# Spezifische Crate-Tests ausführen
cargo test -p bom-core

# Benchmarks ausführen
cargo bench
```

### Dokumentation

```bash
# Dokumentation generieren
cargo doc --no-deps --open

# Dokumentation überprüfen
cargo doc --no-deps --all
```

## 🔧 Projektstruktur

```
bom/
├── crates/
│   ├── bom-core/       # Kerndatenmodelle
│   ├── bom-graph/      # Graph-Datenstruktur
│   ├── bom-calc/       # Berechnungs-Engines
│   ├── bom-cache/      # Caching-Schicht
│   ├── bom-ffi/        # C-FFI-Bindungen
│   ├── bom-adapters/   # ERP/PLM-Adapter
│   └── bom-benches/    # Benchmarks
├── examples/           # Verwendungsbeispiele
├── docs/              # Dokumentation
└── tests/             # Integrationstests
```

## ❓ Fragen?

Wenn Sie Fragen haben:

1. Überprüfen Sie die vorhandene Dokumentation
2. Durchsuchen Sie GitHub Issues
3. Fragen Sie in GitHub Discussions
4. E-Mail an xiaoivan1@proton.me

Vielen Dank für Ihren Beitrag! 🎉
