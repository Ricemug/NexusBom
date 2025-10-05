# BOM Engine Benchmark Results

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

BOM 計算引擎展現出優異的性能：

- 圖結構建構速度極快（微秒級）
- 物料展開計算高效（微秒級）
- 成本計算快速（微秒級）
- Where-Used 反查速度極快（納秒級）

所有核心功能都在微秒到納秒級別完成，能夠支持大規模 BOM 計算需求。
