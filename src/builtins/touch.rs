use std::fs::{write};

pub fn main(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }

    let filename = command.args[0].replace("~", &crate::utils::home_dir());

    write(filename, "").unwrap();
}