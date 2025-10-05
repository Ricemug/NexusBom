# BOM Engine Benchmark-Ergebnisse

[English](../BENCHMARK_RESULTS.md) | [繁體中文](./BENCHMARK_RESULTS.zh-TW.md) | [简体中文](./BENCHMARK_RESULTS.zh-CN.md)

## Leistungsmetriken

### Graph-Konstruktion
- 2 Ebenen × 5 Kinder: **~2,07 µs**
- 3 Ebenen × 4 Kinder: **~1,57 µs**
- 4 Ebenen × 3 Kinder: **~1,19 µs**

### Material-Explosion
- 2 Ebenen × 5 Kinder: **~23,5 µs**
- 3 Ebenen × 4 Kinder: **~19,8 µs**
- 4 Ebenen × 3 Kinder: **~19,1 µs**

### Kostenberechnung
- 2 Ebenen × 5 Kinder: **~27,7 µs**
- 3 Ebenen × 4 Kinder: **~24,7 µs**
- 4 Ebenen × 3 Kinder: **~20,9 µs**

### Where-Used-Analyse
- Einfache 2-Ebenen-Struktur: **~192 ns**

## Zusammenfassung

Die BOM-Berechnungs-Engine zeigt hervorragende Leistung:

- Graph-Konstruktion ist extrem schnell (Mikrosekunden-Ebene)
- Material-Explosionsberechnung ist hocheffizient (Mikrosekunden-Ebene)
- Kostenberechnung ist schnell (Mikrosekunden-Ebene)
- Where-Used-Rückwärtssuche ist blitzschnell (Nanosekunden-Ebene)

Alle Kernfunktionen werden in Mikro- bis Nanosekunden ausgeführt und können großangelegte BOM-Berechnungsanforderungen unterstützen.
