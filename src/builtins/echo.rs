pub fn main(command: crate::parser::Command) {
    for arg in command.args {
        print!("{} ", arg);
    }
    println!();
}