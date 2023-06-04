use std::{collections::HashMap, error::Error};

#[cfg(feature = "dhat-heap")]
use dhat;
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use rust_benchmarking_and_profiling::*;

fn histogram(fields: &[Option<Field>]) -> HashMap<Option<Field>, i64> {
    fields
        .into_iter()
        .cloned()
        .fold(HashMap::new(), |mut acc, f| {
            *acc.entry(f).or_default() += 1;
            acc
        })
}

fn go(input: &str) -> Result<(), Box<dyn Error>> {
    let bytes = read_file(&input.into())?;
    let fields = read_csv(&bytes)?;
    println!("{:#?}", histogram(fields.as_slice()));
    Ok(())
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    go("test.csv").unwrap_or_else(|e| {
        eprintln!("[csv-count] {}", e);
        std::process::exit(1);
    });
}
