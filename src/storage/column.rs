use crate::catalog::schema::DataType;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

pub enum ColumnVault {
    WholeNumbers(Arc<Vec<i32>>),
    MoneyAndScores(Arc<Vec<f64>>),
}

pub struct ColumnManager {
    pub _name: String, 
    pub vault: ColumnVault,
}

impl ColumnManager {
    pub fn load_from_disk(name: &str, dtype: &DataType, filename: &str) -> Result<Self, String> {
        let mut file = File::open(filename).map_err(|e| format!("Failed to open {}: {}", filename, e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        let vault = match dtype {
            DataType::Int32 => {
                let ptr = buffer.as_ptr() as *const i32;
                let len = buffer.len() / 4;
                let data = unsafe { std::slice::from_raw_parts(ptr, len).to_vec() };
                ColumnVault::WholeNumbers(Arc::new(data))
            }
            DataType::Float64 => {
                let ptr = buffer.as_ptr() as *const f64;
                let len = buffer.len() / 8;
                let data = unsafe { std::slice::from_raw_parts(ptr, len).to_vec() };
                ColumnVault::MoneyAndScores(Arc::new(data))
            }
        };

        Ok(Self { _name: name.to_string(), vault })
    }
}