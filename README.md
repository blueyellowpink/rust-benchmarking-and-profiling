# Benchmarking and Profiling

## Book
 - https://nnethercote.github.io/perf-book

## Set RUSTFLAGS
```bash
set -x RUSTFLAGS "-C target-cpu=native"
```

## Build
```bash
cargo build --release # optimized version
cargo build --release --features unoptimized # un-optimized version
```

## Perf
- Allow `perf` to do its profiling
```bash
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

- Profiling
```bash
perf stat -ad -r 100 target/release/rust-benchmarking-and-profiling
```

- Set cat /proc/sys/kernel/kptr_restrict to 0
```bash
echo 0 > /proc/sys/kernel/kptr_restrict # or
sudo vim /proc/sys/kernel/kptr_restrict
```

- Record and report events
```bash
perf record -e L1-dcache-loads,LLC-load-misses --call-graph dwarf -- target/release/rust-benchmarking-and-profiling
perf report
```

## Flamegraph
- Inferno
```bash
cargo install inferno
perf script | inferno-collapse-perf > stacks.folded
cat stacks.folded | inferno-flamegraph > profile-inferno.svg
```

- Flamegraph
```bash
cargo install flamegraph
flamegraph -o profile-flamegraph.svg -- target/release/rust-benchmarking-and-profiling
```

## Hyperfine
- Install
```bash
cargo install hyperfine
```

- Benchmark between debug and release build
```bash
./hyperfine.sh
```

- Benchmark between optimized and unoptimized of release build
```bash
./hyperfine_opt.sh
```

## Heap memory profiler
- Bytehound: https://github.com/koute/bytehound
- Heaptrack: https://github.com/KDE/heaptrack

## DHAT
- Run DHAT with for optimized and unoptimized version of code (make sure to set `lto = false` in `Cargo.toml`)
```bash
cargo run --profile release-dhat --features dhat-heap # DHAT optimized version
cargo run --profile release-dhat --features dhat-heap,unoptimized # DHAT un-optimized version
```

- View DHAT output with: https://nnethercote.github.io/dh_view/dh_view.html

## Micro-benchmark with Criterion
```bash
cargo bench # bench the optimized read_csv function
cargo bench --features unoptimized # bench the unoptimized read_csv function
```

## Trace syscall with `strace`
```bash
strace target/release/rust-benchmarking-and-profiling
```
