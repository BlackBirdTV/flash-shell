use std::fs::{create_dir_all};

pub fn main(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m",
            command.args.len()
        );
        return;
    }
    create_dir_all(command.args[0].clone()).unwrap();
}