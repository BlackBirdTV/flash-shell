use std::fs::{read_dir};

pub fn main(command: crate::parser::Command) {
    let dir;
    if command.args.len() < 1 {
        dir = read_dir("./").expect("Unable to read current directories contents");
    }
    else {
        let filename = command.args[0].replace("~", &crate::utils::home_dir());
        let path = std::path::Path::new(&filename);

        if !path.exists() {
            println!("\x1b[31mDirectory \"{}\" does not exist.\x1b[0m\r",
                command.args[0].clone()
            );
            return;
        }

        dir = read_dir(filename).expect("Unable to read directories contents");
    }
    for path in dir {
        let path = path.unwrap().path();
        let name = path.display().to_string();
        println!("{}{}\r", if command.args.len() < 1 { name[2..].to_string() } else { name }, if path.is_dir() { "/" } else { "" })
    }
}