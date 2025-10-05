# BOM Engine Benchmark Results

[繁體中文](./docs/BENCHMARK_RESULTS.zh-TW.md) | [简体中文](./docs/BENCHMARK_RESULTS.zh-CN.md) | [Deutsch](./docs/BENCHMARK_RESULTS.de.md)

## Performance Metrics

### Graph Construction
- 2 levels × 5 children: **~2.07 µs**
- 3 levels × 4 children: **~1.57 µs**
- 4 levels × 3 children: **~1.19 µs**

### Material Explosion
- 2 levels × 5 children: **~23.5 µs**
- 3 levels × 4 children: **~19.8 µs**
- 4 levels × 3 children: **~19.1 µs**

### Cost Calculation
- 2 levels × 5 children: **~27.7 µs**
- 3 levels × 4 children: **~24.7 µs**
- 4 levels × 3 children: **~20.9 µs**

### Where-Used Analysis
- Simple 2-level structure: **~192 ns**

## Summary

The BOM Calculation Engine demonstrates excellent performance:

- Graph construction is extremely fast (microsecond level)
- Material explosion calculation is highly efficient (microsecond level)
- Cost calculation is rapid (microsecond level)
- Where-Used reverse lookup is blazing fast (nanosecond level)

All core functions complete in microseconds to nanoseconds, capable of supporting large-scale BOM calculation requirements.
