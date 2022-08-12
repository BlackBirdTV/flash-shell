use std::io::{self, Write, Stdout};
use std::thread;

use crate::parser::CommandAction;

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::FollowCommand(cmd) => {
            clear(command);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            clear(command);
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => clear(command)
    }
}

fn clear(command: crate::parser::Command) {
    if command.args.len() != 0 {
        println!("\x1b[31mExpected 0 arguments but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}