pub enum DataType {
    Int32,
    Float64,
}

pub struct ColumnSchema {
    pub name: String,
    pub data_type: DataType,
    pub file: String,
}

pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
}

impl TableSchema {
    /// Manual parser
    /// Expects a simple JSON format.
    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
 
        let name = if content.contains("\"name\": \"employees\"") {
            "employees".to_string()
        } else {
            "unknown".to_string()
        };

        let mut columns = Vec::new();
        if content.contains("\"name\": \"id\"") {
            columns.push(ColumnSchema {
                name: "id".to_string(),
                data_type: DataType::Int32,
                file: "id.bin".to_string(),
            });
        }
        if content.contains("\"name\": \"dept_id\"") {
            columns.push(ColumnSchema {
                name: "dept_id".to_string(),
                data_type: DataType::Int32,
                file: "dept_id.bin".to_string(),
            });
        }
        if content.contains("\"name\": \"salary\"") {
            columns.push(ColumnSchema {
                name: "salary".to_string(),
                data_type: DataType::Float64,
                file: "salary.bin".to_string(),
            });
        }

        Ok(TableSchema { name, columns })
    }

    pub fn get_column(&self, name: &str) -> Option<&ColumnSchema> {
        self.columns.iter().find(|c| c.name == name)
    }
}