mod parser;

use std::io::{self, Write};
use std::env::{current_dir, set_current_dir};
use std::path::Path;

use parser::{parse, Command};

fn main() {
    
    let green;
    let blue;
    let red;
    let reset;

    // Enable ANSI Support for the old Windows Shell. If it fails, disable ANSI Colors.
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            green = "\x1b[92m";
            blue = "\x1b[94m";
            red = "\x1b[31m";
            reset = "\x1b[0m";
        }
        Err(_) => {
            green = "";
            blue = "";
            red = "";
            reset = "";
        }
    }

    // Set up STDIN
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    loop {
        let user = whoami::username();
        let pc_name = whoami::hostname();
        let path = current_dir().expect("Invalid Path");

        // Read input from user
        print!("{green}{user}@{pc_name}{reset}:{blue}{path}{reset}↯ ", path = path.into_os_string().to_str().unwrap_or("?"));
        io::stdout().flush().unwrap();
        stdin.read_line(&mut buffer).expect("Failed to read from stdin");

        let command = parse((&buffer[..buffer.len()-1]).to_owned());

        match command.action.as_str() {
            "cd" => if command.args.len() > 0{
                let path = Path::new(&command.args[0]);
                println!("{}",  path.display());
                set_current_dir(&path).expect("Unable to change into ");
            },
            _ => {
                println!("{red}Unknown command: {}", command.action.as_str().replace(" ", "_").replace("\n", "-"));
            }
        }

        // Flush buffer
        buffer = String::new();
    }

    println!("{reset}");
}
