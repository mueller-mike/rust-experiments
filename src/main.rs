mod experiments;

use experiments::{hello};

use std::io;

fn main() {
    loop {
        println!("[SYSTEM] Welcome to the Experiments program!");
        println!("[SYSTEM] 1. Hello");
        println!("[SYSTEM] 2. Exit");
        println!("[SYSTEM] Please choose a program by entering the corresponding number or name:");

        let mut program_selection = String::new();
        io::stdin().read_line(&mut program_selection).expect("Failed to read input");
        let program_selection = program_selection.trim();

        match program_selection {
            "1" => hello::run(),
            "2" => {
                println!("[SYSTEM] Goodbye!");
                break;
            }
            _ => println!("[SYSTEM] Invalid choice."),
        }
    }
}