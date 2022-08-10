pub fn main(command: crate::parser::Command) {
    if command.args.len() != 0 {
        println!("\x1b[31mExpected 0 arguments but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }
    println!("{}\r", std::env::current_dir().unwrap().into_os_string().to_str().unwrap_or(""))
}
