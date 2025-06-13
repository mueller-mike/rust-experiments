use std::io;
use std::thread;
use std::time::Duration;

pub fn run() {
    thread::sleep(Duration::from_millis(500));
    println!("[SYSTEM] Starting Calculator");
    thread::sleep(Duration::from_secs(1));
    println!("Welcome to Calculator!");
    println!("Available Operations:");
    println!("1. Addition");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Quit");

    loop {
        println!("Please enter your desired operation:");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim().to_lowercase();

        match command.as_str() {
            "quit" => {
                thread::sleep(Duration::from_millis(500));
                println!("[SYSTEM] Calculator Finished Execution");
                break;
            }
            "addition" | "1" | "subtract" | "2" | "multiply" | "3" | "divide" | "4" => {
                println!("Please enter x value:");
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read line");
                let x: i32 = match x.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number for x.");
                        continue;
                    }
                };

                println!("Please enter y value:");
                let mut y = String::new();
                io::stdin().read_line(&mut y).expect("Failed to read line");
                let y: i32 = match y.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number for y.");
                        continue;
                    }
                };

                match command.as_str() {
                    "addition" | "1" => {
                        addition(x, y);
                    }
                    "subtract" | "2" => {
                        subtract(x, y);
                    }
                    "multiply" | "3" => {
                        multiply(x, y);
                    }
                    "divide" | "4" => {
                        divide(x, y);
                    }
                    _ => {
                        println!("Invalid command.");
                    }
                }
            }
            _ => {
                println!("Unknown command: '{}'", command);
            }
        }
    }
}

fn addition(x: i32, y: i32) {
    let z: i32 = x + y;
    println!("{} + {} = {}", x, y, z);
}

fn subtract(x: i32, y: i32) {
    let z: i32 = x - y;
    println!("{} - {} = {}", x, y, z);
}

fn multiply(x: i32, y: i32) {
    let z: i32 = x * y;
    println!("{} * {} = {}", x, y, z);
}

fn divide(x: i32, y: i32) {
    let z: f32 = x as f32 / y as f32;
    println!("{} / {} = {}", x, y, z);
}
