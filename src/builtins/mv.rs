pub fn main(command: crate::parser::Command) {
    if command.args.len() != 2 {
        println!("\x1b[31mExpected 2 arguments but received {}\x1b[0m",
            command.args.len()
        );
        return;
    }

    let path = std::path::Path::new(&command.args[0]);

    if !path.exists() {
        println!("\x1b[31mDirectory \"{}\" does not exist.\x1b[0m",
            command.args[0].clone()
        );
        return;
    }

    let path = std::path::Path::new(&command.args[1]);

    if !path.exists() {
        println!("\x1b[31mDirectory \"{}\" does not exist.\x1b[0m",
            command.args[1].clone()
        );
        return;
    }

    std::fs::rename(command.args[0].clone(), command.args[1].clone()).unwrap();
}