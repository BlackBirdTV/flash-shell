pub fn main(command: crate::parser::Command) {
    let len = command.args.len();
    for (i, arg) in command.args.iter().enumerate() {
        print!("{}", arg);
        if i < len - 1 {
            print!(" ");
        }
    }
    if !command.contains_flag("c") { println!("\r"); }
}