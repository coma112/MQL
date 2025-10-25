use crate::database::Database;

#[derive(Debug)]
pub struct Context {
    pub current_database: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            current_database: None,
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Create(CreateType),
    Drop(DropType),
    Use(String),
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
                    return Err("Használat: LÉTREHOZ TÁBLA/ADATBÁZIS <név>".to_string());
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

            "HASZNÁL" => {
                if parts.len() < 2 {
                    return Err("Használat: HASZNÁL <adatbázis_név>".to_string());
                }
                Ok(Command::Use(parts[1].to_string()))
            }

            "HELP" | "SEGÍTSÉG" => Ok(Command::Help),

            "EXIT" | "QUIT" | "KILÉP" => Ok(Command::Exit),

            _ => Err(format!("Ismeretlen parancs: '{}'", parts[0])),
        }
    }

    pub fn execute(&self, context: &mut Context) -> Result<(), String> {
        match self {
            Command::Create(create_type) => {
                match create_type {
                    CreateType::Table(name) => {
                        let db_name = context.current_database.as_ref()
                            .ok_or("Nincs kiválasztva adatbázis! Használd: HASZNÁL <adatbázis>")?;

                        let mut db = Database::load(db_name)?;
                        db.create_table(name.clone())?;

                        println!("✓ Tábla létrehozva: {}", name);
                        Ok(())
                    }
                    CreateType::Database(name) => {
                        let db = Database::new(name.clone());
                        db.save()?;

                        println!("✓ Adatbázis létrehozva: {}", name);
                        Ok(())
                    }
                }
            }

            Command::Drop(drop_type) => {
                match drop_type {
                    DropType::Table(name) => {
                        let db_name = context.current_database.as_ref()
                            .ok_or("Nincs kiválasztva adatbázis!")?;

                        let mut db = Database::load(db_name)?;
                        db.drop_table(name)?;

                        println!("✓ Tábla törölve: {}", name);
                        Ok(())
                    }
                    DropType::Database(name) => {
                        Database::delete(name)?;

                        if context.current_database.as_ref() == Some(name) {
                            context.current_database = None;
                        }

                        println!("✓ Adatbázis törölve: {}", name);
                        Ok(())
                    }
                }
            }

            Command::Use(db_name) => {
                // Ellenőrizzük, hogy létezik-e az adatbázis
                Database::load(db_name)?;
                context.current_database = Some(db_name.clone());
                println!("✓ Adatbázis kiválasztva: {}", db_name);
                Ok(())
            }

            Command::Help => {
                println!("\nElérhető MQL parancsok:");
                println!("  LÉTREHOZ TÁBLA <név>       - Tábla létrehozása");
                println!("  LÉTREHOZ ADATBÁZIS <név>   - Adatbázis létrehozása");
                println!("  LEDOB TÁBLA <név>          - Tábla törlése");
                println!("  LEDOB ADATBÁZIS <név>      - Adatbázis törlése");
                println!("  HASZNÁL <adatbázis>        - Adatbázis kiválasztása");
                println!("  HELP                       - Súgó");
                println!("  EXIT                       - Kilépés\n");
                Ok(())
            }

            Command::Exit => {
                println!(":(");
                Ok(())
            }
        }
    }
}