//! This is a simple commandline program designed to merge multiple updated CTestCostData.txt files.
//!
//! `CTestCostData.txt` files are an internal implemtation detail of CMake, so this is not
//! guaranteed to work forever, but it has been stable for at least the last 3 years.
//! The workflow this program is intended for, is approximatly as follows
//!
//! 1. You have many long-running tests, which you split up into subsets and run each subset
//!    on a different machine. On each machine you start with the same `CTestCostData.txt`, and
//!    each machine is assumed to be roughly equally powerful.
//! 2. After all tests have finished you want to merge all the updated Cost results back into one
//!    one file, so that the next time you run tests you have up-to-date costs.
//!
//! This is where this tool kicks in and allows you to merge updated cost files back into one
//! output cost file.

use anyhow::{Context, Result};
use clap::Parser;
use csv::{ReaderBuilder, WriterBuilder};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct CostRecord {
    test_name: String,
    count: usize,
    // The cost is a float, but we don't need to perform any calculations, so just keep
    // it as a string.
    cost: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The base CTestCostFile.txt
    pub(crate) original: PathBuf,
    #[arg(short, long)]
    pub(crate) updates: Vec<PathBuf>,
    #[arg(short, long)]
    pub(crate) output: PathBuf,
}

fn read_updated_record_file(p: &Path) -> Result<Vec<CostRecord>> {
    let updated_costs =
        std::fs::read_to_string(p).with_context(|| format!("failed to read {:?}", p))?;
    let trimmed = if let Some(end_of_records) = updated_costs.rfind("---") {
        &updated_costs[0..end_of_records]
    } else {
        &updated_costs
    };
    let mut reader = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_reader(trimmed.as_bytes());
    let records: Vec<CostRecord> = reader
        .deserialize()
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("failed to deserialize record from file {:?}", p))?;
    Ok(records)
}

fn merge_cost_updates(args: &Cli) -> Result<()> {
    let original_file = std::fs::read_to_string(&args.original)
        .with_context(|| format!("failed to read {:?}", &args.original))?;
    let trimmed = if let Some(end_of_records) = original_file.rfind("---") {
        &original_file[0..end_of_records]
    } else {
        &original_file
    };
    let mut original_reader = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_reader(trimmed.as_bytes());

    let mut record_map = IndexMap::new();
    for res in original_reader.deserialize() {
        let record: CostRecord = res.with_context(|| {
            format!(
                "Failed to deserialise record from file: {:?}",
                args.original
            )
        })?;
        let res = record_map.insert(record.test_name.clone(), record);
        assert!(res.is_none(), "Record for test was already present");
    }
    for update_file in &args.updates {
        let records = read_updated_record_file(update_file)
            .with_context(|| format!("Failed to read update file: {:?}", update_file))?;
        for record in records {
            if let Some(old) = record_map.get(&record.test_name) {
                if record.count > old.count {
                    record_map.insert(record.test_name.clone(), record);
                }
            } else {
                record_map.insert(record.test_name.clone(), record);
            }
        }
    }
    let mut writer_builder = WriterBuilder::new();
    writer_builder.delimiter(b' ').has_headers(false);
    let mut csv_writer = writer_builder.from_path(&args.output)?;

    for (_test_name, record) in record_map.into_iter() {
        csv_writer.serialize(record)?;
    }
    drop(csv_writer);
    let mut cost_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&args.output)?;
    write!(&mut cost_file, "---")
        .with_context(|| format!("Failed to write terminator to {cost_file:?}"))?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    merge_cost_updates(&cli)
}
