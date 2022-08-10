use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }
    
    let filename = command.args[0].replace("~", &crate::utils::home_dir());

    {
        let path = std::path::Path::new(&filename);

        if !path.exists() {
            println!("\x1b[31mFile or Directory \"{}\" does not exist.\x1b[0m\r",
                command.args[0].clone()
            );
            return;
        }
    }

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    for line in &lines[if lines.len() >= 10 { lines.len()-10 } else { 0 }..] {
        println!("{}\r", line)
    }
}