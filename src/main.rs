mod parser;
mod builtins;

use std::io::{self, Write};
use std::env::current_dir;
use parser::{parse};

fn main() {
    let green = "\x1b[92m";
    let blue = "\x1b[94m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    // Enable ANSI Support for the old Windows Shell. If it fails, disable ANSI Colors.
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            
        }
        Err(_) => {
            
        }
    }

    // Set up STDIN
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        let user = whoami::username();
        let pc_name = whoami::hostname();
        let path = current_dir().expect("Invalid Path");

        // Read input from user
        print!("{bold}{green}{user}@{pc_name}{reset}:{bold}{blue}{path}{reset}↯ ", path = path.into_os_string().to_str().unwrap_or("?").replace(&format!("/home/{user}"), "~"));
        flush();
        stdin.read_line(&mut buffer).expect("Failed to read from stdin");

        let command = parse((&buffer[..buffer.len()-1]).to_owned());

        match command.action.as_str() {
            "echo" => builtins::ECHO(command),
            "cd" => builtins::CD(command),
            "cp" => builtins::CP(command),
            "mkdir" => builtins::MKDIR(command),
            "touch" => builtins::TOUCH(command),
            "mv" => builtins::MV(command),
            "clear" => builtins::CLEAR(command),
            "ls"=> builtins::LS(command),
            "info" => builtins::INFO(command),
            "pwd" => builtins::PWD(command),
            "rm" => builtins::RM(command),
            "exit" => {break;}
            _ => {
                if std::path::Path::exists(&std::path::Path::new(&format!("./exts/{}", command.action))) {
                    println!("EXTENSION FOUND");
                }
                else {
                    println!("{red}Unknown command: {}", command.action.as_str().replace(" ", "_").replace("\n", "-"));
                }
            }
        }

        // Flush buffer
        buffer = String::new();
    }

    print!("{reset}");
    flush();
}

fn flush() {
    io::stdout().flush().unwrap()
}
