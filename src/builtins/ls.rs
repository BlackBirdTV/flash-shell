use std::fs::{read_dir};

pub fn main(command: crate::parser::Command) {
    let dir;
    if command.args.len() < 1 {
        dir = read_dir("./").expect("Unable to read current directories contents");
    }
    else {
        let path = std::path::Path::new(&command.args[0]);

        if !path.exists() {
            println!("\x1b[31mDirectory \"{}\" does not exist.\x1b[0m",
                command.args[0].clone()
            );
            return;
        }

        dir = read_dir(command.args[0].clone()).expect("Unable to read directories contents");
    }
    for path in dir {
        let path = path.unwrap().path();
        let name = path.display().to_string();
        println!("{}{}", name[2..].to_string(), if path.is_dir() { "/" } else { "" })
    }
}