mod experiments;

use experiments::calculator;
use experiments::hello;

use std::io;

fn main() {
    
    println!("[SYSTEM] Welcome to the Rust-Experiments program!");
    
    loop {
        print_available_programs();
        
        let mut program_selection = String::new();
        io::stdin()
            .read_line(&mut program_selection)
            .expect("Failed to read input");
        let program_selection = program_selection.trim().to_lowercase();

        match program_selection.as_str() {
            "1" | "hello" => hello::run(),
            "2" | "calculator" => calculator::run(),
            "3" | "quit" => {
                println!("[SYSTEM] Rust-Experiments finished execution!");
                break;
            }
            "help" => print_available_programs(),
            _ => println!("[SYSTEM] Invalid choice."),
        }
    }
}

fn print_available_programs() {
    println!("[SYSTEM] Available programs:");
    println!("[SYSTEM] 1. Hello");
    println!("[SYSTEM] 2. Calculator");
    println!("[SYSTEM] 3. Quit");
    println!("[SYSTEM] Please choose a program by entering the corresponding number or name:");
}