# Benchmarking and Profiling

## Hyperfine
- Install
```bash
cargo install hyperfine
```

- Benchmark between debug and release build
```bash
./hyperfine.sh
```

## DHAT
- Run DHAT with for optimized and unoptimized version of code (make sure to set `lto = false` in `Cargo.toml`)
```bash
cargo run --release --features dhat-heap # optimized version
cargo run --release --features dhat-heap,unoptimized # un-optimized version
```

- View DHAT output with: https://nnethercote.github.io/dh_view/dh_view.html

- Benchmark between optimized and unoptimized of release build
```bash
./hyperfine_opt.sh
```

## Micro-benchmark with Criterion
```bash
cargo bench # bench the optimized read_csv function
cargo bench --features unoptimized # bench the unoptimized read_csv function
```
