# Bench report

## Setup

- Host: Linux klaim 6.14.0-37-generic x86_64 (Ubuntu 24.04)
- Rust: rustc 1.91.1 (ed61e7d7e 2025-11-07)
- Criterion: 0.5.1

## Command

```
cargo bench --bench vec_capacity --features bench-string -- --save-baseline base
```

## Results

| Group | Case | n | Estimate (us) | Range (us) |
| --- | --- | --- | --- | --- |
| vec_capacity | no_cap | 100000 | 53.995 | 53.739 - 54.258 |
| vec_capacity | with_cap | 100000 | 52.775 | 52.465 - 53.081 |
| string_capacity | no_cap | 1000 | 3.4975 | 3.4547 - 3.5425 |
| string_capacity | with_cap | 1000 | 2.1276 | 2.1060 - 2.1540 |

## Conclusion

For `Vec<u64>`, pre-allocating with `Vec::with_capacity` is about 2% faster here, likely due to fewer reallocations while pushing 100k elements. For `String`, `with_capacity` is about 39% faster with the fixed chunk and repeat count. The relative gains should grow with larger `n` and shrink for tiny workloads where allocator costs are less visible.
