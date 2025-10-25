#[derive(Debug)]
pub enum Command {
    Create(CreateType),
    Drop(DropType),
    Help,
    Exit,
}

#[derive(Debug)]
pub enum CreateType {
    Table(String),
    Database(String),
}

#[derive(Debug)]
pub enum DropType {
    Table(String),
    Database(String),
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, String> {
        let input = input.trim().to_uppercase();
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return Err("Üres parancs!".to_string());
        }

        match parts[0] {
            "LÉTREHOZ" => {
                if parts.len() < 3 {
                    return Err("Használat: LÉTREHOZ TÁBLA/ADATBÁZIS/ <név>".to_string());
                }

                let name = parts[2].to_string();

                let create_type = match parts[1] {
                    "TÁBLA" => CreateType::Table(name),
                    "ADATBÁZIS" => CreateType::Database(name),
                    _ => return Err(format!("Ismeretlen LÉTREHOZ típus: {}", parts[1])),
                };

                Ok(Command::Create(create_type))
            }

            "LEDOB" => {
                if parts.len() < 3 {
                    return Err("Használat: LEDOB TÁBLA/ADATBÁZIS <név>".to_string());
                }

                let name = parts[2].to_string();

                let drop_type = match parts[1] {
                    "TÁBLA" => DropType::Table(name),
                    "ADATBÁZIS" => DropType::Database(name),
                    _ => return Err(format!("Ismeretlen LEDOB típus: {}", parts[1])),
                };

                Ok(Command::Drop(drop_type))
            }

            "HELP" => Ok(Command::Help),

            "EXIT" | "QUIT" => Ok(Command::Exit),

            _ => Err(format!("Ismeretlen parancs: '{}'", parts[0])),
        }
    }

    pub fn execute(&self) {
        match self {
            Command::Create(create_type) => {
                match create_type {
                    CreateType::Table(name) => {
                        println!("Tábla létrehozva: {}", name);
                    }
                    CreateType::Database(name) => {
                        println!("Adatbázis létrehozva: {}", name);
                    }
                }
            }

            Command::Drop(drop_type) => {
                match drop_type {
                    DropType::Table(name) => {
                        println!("Tábla törölve: {}", name);
                    }
                    DropType::Database(name) => {
                        println!("Adatbázis törölve: {}", name);
                    }
                }
            }

            Command::Help => {
                println!("\nElérhető MQL parancsok:");
                println!("  CREATE TABLE <név>     - Tábla létrehozása");
                println!("  CREATE DATABASE <név>  - Adatbázis létrehozása");
                println!("  DROP TABLE <név>       - Tábla törlése");
                println!("  DROP DATABASE <név>    - Adatbázis törlése");
                println!("  HELP                   - Súgó");
                println!("  EXIT                   - Kilépés\n");
            }

            Command::Exit => {
                println!(":(");
            }
        }
    }
}