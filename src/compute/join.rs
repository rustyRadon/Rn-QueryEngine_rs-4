use std::collections::HashMap;
use crate::util::bitmask::BitMask;

pub struct DeptLookup {
    pub map: HashMap<i32, String>,
}

impl DeptLookup {
    pub fn new(data: Vec<(i32, String)>) -> Self {
        let mut map = HashMap::new();
        for (id, name) in data {
            map.insert(id, name);
        }
        Self { map }
    }

    pub fn probe(&self, ids: &[i32], mask: &BitMask) -> Vec<String> {
        let mut results = Vec::with_capacity(mask.count_active());
        
        for (i, &id) in ids.iter().enumerate() {
            if mask.get(i) {
                let name = self.map.get(&id).cloned().unwrap_or_else(|| "Unknown".to_string());
                results.push(name);
            }
        }
        results
    }

    pub fn from_json(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut data = Vec::new();

        let blocks: Vec<&str> = content.split('}').collect();
        for block in blocks {
            if !block.contains("\"id\"") { continue; }
            
            let id_part = block.split("\"id\":").nth(1).unwrap_or("");
            let id_str = id_part.split(',').next().unwrap_or("").trim();
            let id: i32 = id_str.parse().unwrap_or(0);

            let name_part = block.split("\"name\":").nth(1).unwrap_or("");
            let name = name_part.split('\"').nth(1).unwrap_or("Unknown").to_string();

            data.push((id, name));
        }

        Ok(Self::new(data))
    }
}