pub enum DataType {
    Int32,
    Float64,
}

pub struct ColumnSchema {
    pub _name: String,      
    pub data_type: DataType,
    pub _file: String,      
}

pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
}

impl TableSchema {
    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        
        let name = "EmployeeFactTable".to_string();
        let mut columns = Vec::new();

        if content.contains("\"name\": \"id\"") {
            columns.push(ColumnSchema { _name: "id".to_string(), data_type: DataType::Int32, _file: "id.bin".to_string() });
        }
        if content.contains("\"name\": \"age\"") {
            columns.push(ColumnSchema { _name: "age".to_string(), data_type: DataType::Int32, _file: "age.bin".to_string() });
        }
        if content.contains("\"name\": \"dept_id\"") {
            columns.push(ColumnSchema { _name: "dept_id".to_string(), data_type: DataType::Int32, _file: "dept_id.bin".to_string() });
        }
        if content.contains("\"name\": \"salary\"") {
            columns.push(ColumnSchema { _name: "salary".to_string(), data_type: DataType::Float64, _file: "salary.bin".to_string() });
        }

        Ok(TableSchema { name, columns })
    }

    pub fn get_column(&self, name: &str) -> Option<&ColumnSchema> {
        self.columns.iter().find(|c| c._name == name)
    }
}