use std::fs::{copy};

pub fn main(command: crate::parser::Command) {
    if command.args.len() != 2 {
        println!("\x1b[31mExpected 2 arguments but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }

    let path0 = command.args[0].replace("~", &crate::utils::home_dir());
    let path = std::path::Path::new(&path0);

    if !path.exists() {
        println!("\x1b[31mFile or Directory \"{}\" does not exist.\x1b[0m\r",
            command.args[0].clone()
        );
        return;
    }

    let path1 = command.args[1].replace("~", &crate::utils::home_dir());

    copy(path0, path1).unwrap();
}