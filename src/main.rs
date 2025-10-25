mod commands;

use std::io::{self, Write};
use commands::Command;

fn main() {
    println!("=== MQL CONSOLE ===");

    loop {
        print!("sql> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match Command::parse(&input) {
            Ok(command) => {
                command.execute();

                if matches!(command, Command::Exit) {
                    break;
                }
            }
            Err(error) => {
                println!("❌ Hiba: {}", error);
            }
        }
    }
}
