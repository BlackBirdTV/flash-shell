use std::io::{self, Write};

pub fn main(command: crate::parser::Command) {
    if command.args.len() != 0 {
        println!("\x1b[31mExpected 0 arguments but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}