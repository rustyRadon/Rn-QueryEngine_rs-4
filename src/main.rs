mod catalog;
mod storage;
mod compute;
mod util;

use crate::catalog::schema::TableSchema;
use crate::storage::column::{ColumnManager, ColumnVault};
use crate::compute::join::DeptLookup;
use crate::compute::functions::crunch_float_sum;
use std::env;
use std::time::Instant;
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let schema = TableSchema::from_file("metadata.json")?;
    let lookup = DeptLookup::from_json("departments.json")?;

    let dept_col = schema.get_column("dept_id").ok_or("Dept ID missing")?;
    let sal_col = schema.get_column("salary").ok_or("Salary missing")?;
    let age_col = schema.get_column("age").ok_or("Age missing")?;

    let dept_m = ColumnManager::load_from_disk(&dept_col._name, &dept_col.data_type, &dept_col._file)?;
    let sal_m = ColumnManager::load_from_disk(&sal_col._name, &sal_col.data_type, &sal_col._file)?;
    let age_m = ColumnManager::load_from_disk(&age_col._name, &age_col.data_type, &age_col._file)?;

    let args: Vec<String> = env::args().collect();
    let min_salary = args.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(100000.0);
    let target_dept = args.get(2).and_then(|s| s.parse::<i32>().ok());
    let min_age = args.get(3).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);

    println!("--- Relational Engine: {} ---", schema.name);
    println!("Query: Salary > ${:.2}, Min Age: {}", min_salary, min_age);

    let start_time = Instant::now();

    let mut mask = util::bitmask::BitMask::new(1000000);
    for i in 0..1000000 { mask.set(i); }

    if let ColumnVault::MoneyAndScores(data) = &sal_m.vault {
        for (i, &val) in data.iter().enumerate() {
            if val < min_salary { mask.clear(i); }
        }
    }

    if let ColumnVault::WholeNumbers(data) = &age_m.vault {
        for (i, &val) in data.iter().enumerate() {
            if mask.get(i) && val < min_age { mask.clear(i); }
        }
    }

    if let Some(d_id) = target_dept {
        if let ColumnVault::WholeNumbers(data) = &dept_m.vault {
            for (i, &val) in data.iter().enumerate() {
                if mask.get(i) && val != d_id { mask.clear(i); }
            }
        }
    }

    let total_payroll = crunch_float_sum(&sal_m.vault, &mask);
    
    let mut joined_names = Vec::new();
    if let ColumnVault::WholeNumbers(ids) = &dept_m.vault {
        joined_names = lookup.probe(ids, &mask);
    }

    let duration = start_time.elapsed();

    let mut counts = HashMap::new();
    for name in &joined_names {
        *counts.entry(name).or_insert(0) += 1;
    }

    println!("\n[JOIN RESULTS]");
    for (dept, count) in counts {
        println!("{:<12} : {} employees", dept, count);
    }

    println!("\n-----------------------");
    println!("Total Matches  : {}", joined_names.len());
    println!("Total Payroll  : ${:.2}", total_payroll);
    println!("Execution Time : {:.2?}", duration);

    Ok(())
}