use std::fs::{write};

pub fn main(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m",
            command.args.len()
        );
        return;
    }
    let mut text = String::new();
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    println!(":w to write and exit, :q to discard and exit");
    loop {
        stdin.read_line(&mut buffer).expect("Failed to read from stdin");
        if buffer == ":w\n" {
            break;
        }
        else if buffer == ":q\n" {
            return;
        }
        text = format!("{text}{buffer}");
        buffer = String::new();
    }
    write(command.args[0].clone(), text).unwrap();
}