use std::io::Stdout;
use crate::parser::CommandAction;
use std::thread;
use std::io;

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    if command.args.len() != 0 {
        println!("\x1b[31mExpected 0 arguments but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }
    match command.followed_action.clone() {
        CommandAction::PipeFile(filename) => {
            std::fs::write(filename, get_pwd()).expect("Error occurred while writing to file");
        }
        CommandAction::PipeCommand(cmd) => {
            let mut cmd = *cmd;

            cmd.args = vec![get_pwd().to_owned()];

            crate::run_command(cmd, stdout);
        }
        CommandAction::FollowCommand(cmd) => {
            pwd();
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            pwd();
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => pwd()
    }
}

fn pwd() {
    println!("{}\r", get_pwd())
}

fn get_pwd() -> String {
    let outp = std::env::current_dir().unwrap().into_os_string().to_str().unwrap_or("").to_owned();
    outp
}
