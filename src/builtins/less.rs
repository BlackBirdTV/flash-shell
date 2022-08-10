pub fn main(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }

    let path = command.args[0].replace("~", &crate::utils::home_dir());
    let path = std::path::Path::new(&path);
    
    if !path.exists() {
        println!("\x1b[31mFile \"{}\" doesn't exist.\x1b[0m\r",
            path.file_name().unwrap().to_str().unwrap()
        );
        return;
    }

    if path.is_dir() {
        println!("\x1b[31mObject \"{}\" is a directory.\x1b[0m\r",
            path.file_name().unwrap().to_str().unwrap()
        );
        return;
    }

    for line in std::fs::read_to_string(path).unwrap_or(String::new()).lines() { println!("{}\r", line); }
}