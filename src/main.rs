use clearscreen::clear;
use std::io;
use std::io::Write;
use std::time::Duration;
use std::{thread, time};

fn main() {
    // Clear the screen
    clear().unwrap();

    println!("\n+---------------------=+");
    println!("| Basic Calculator App |");
    println!("+----------------------+\n\n");
    loop {
        println!("Select an operation:\n");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit\n");

        print!("Selection: ");
        io::stdout().flush().expect("Failed to flush");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid input. Please enter a number.");
                timeout(1000);
                clear().unwrap();
                continue;
            }
        };

        if choice == 5 {
            println!("\nExiting the calculator.");
            timeout(1000);
            clear().unwrap();
            break;
        }

        if choice < 1 || choice > 4 {
            println!("\nInvalid choice.");
            timeout(1000);
            continue;
        }

        let (op, operator) = match choice {
            1 => ("Addition", "+"),
            2 => ("Subtraction", "-"),
            3 => ("Multiplication", "*"),
            4 => ("Division", "/"),
            _ => unreachable!(),
        };

        println!("\n{}:", op);

        print!("Enter the first number: ");
        io::stdout().flush().expect("Failed to flush");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");

        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid input. Please enter valid numbers.");
                timeout(1000);
                clear().unwrap();
                continue;
            }
        };

        print!("Enter the second number: ");
        io::stdout().flush().expect("Failed to flush");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");

        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid input. Please enter valid numbers.");
                timeout(1000);
                clear().unwrap();
                continue;
            }
        };

        let result = match choice {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => {
                if num2 == 0.0 {
                    println!("\nError: Cannot divide by zero.");
                    timeout(1000);
                    clear().unwrap();
                    continue;
                }
                num1 / num2
            }
            _ => unreachable!(),
        };

        println!("\n{}: {} {} {} = {}\n\n", op, num1, operator, num2, result);
        println!("\nPress any key to continue.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        clear().unwrap();
    }
}

fn timeout(time_in_millis: u64) {
    let timeout: Duration = time::Duration::from_millis(time_in_millis);
    let now = time::Instant::now();

    thread::sleep(timeout);
    assert!(now.elapsed() >= timeout);
}
