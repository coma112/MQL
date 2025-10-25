mod commands;
mod database;

use std::io::{self, Write};
use commands::{Command, Context};

fn main() {
    println!("=== MQL CONSOLE ===");
    let mut context = Context::new();

    loop {
        print!("sql> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match Command::parse(&input) {
            Ok(command) => {
                if matches!(command, Command::Exit) {
                    println!(":(");
                    break;
                }

                if let Err(e) = command.execute(&mut context) {
                    println!("❌ Hiba: {}", e);
                }
            }
            Err(error) => {
                println!("❌ Hiba: {}", error);
            }
        }
    }
}