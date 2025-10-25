use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(name: String) -> Self {
        Database {
            name,
            tables: HashMap::new(),
        }
    }

    pub fn load(name: &str) -> Result<Self, String> {
        let path = format!("databases/{}.json", name);

        if !Path::new(&path).exists() {
            return Err(format!("Az adatbázis nem létezik: {}", name));
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Fájl olvasási hiba: {}", e))?;

        let db: Database = serde_json::from_str(&content)
            .map_err(|e| format!("JSON parse hiba: {}", e))?;

        Ok(db)
    }

    pub fn save(&self) -> Result<(), String> {
        fs::create_dir_all("databases")
            .map_err(|e| format!("Könyvtár létrehozási hiba: {}", e))?;

        let path = format!("databases/{}.json", self.name);
        let json = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("JSON serialization hiba: {}", e))?;

        fs::write(&path, json)
            .map_err(|e| format!("Fájl írási hiba: {}", e))?;

        Ok(())
    }

    pub fn create_table(&mut self, name: String) -> Result<(), String> {
        if self.tables.contains_key(&name) {
            return Err(format!("A tábla már létezik: {}", name));
        }

        let table = Table {
            name: name.clone(),
            columns: Vec::new(),
            rows: Vec::new(),
        };

        self.tables.insert(name, table);
        self.save()?;

        Ok(())
    }

    pub fn drop_table(&mut self, name: &str) -> Result<(), String> {
        if !self.tables.contains_key(name) {
            return Err(format!("A tábla nem létezik: {}", name));
        }

        self.tables.remove(name);
        self.save()?;

        Ok(())
    }

    pub fn delete(name: &str) -> Result<(), String> {
        let path = format!("databases/{}.json", name);

        if !Path::new(&path).exists() {
            return Err(format!("Az adatbázis nem létezik: {}", name));
        }

        fs::remove_file(&path)
            .map_err(|e| format!("Törlési hiba: {}", e))?;

        Ok(())
    }
}