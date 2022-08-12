use std::fs::{write};

use std::{io::Stdout};

use crate::parser::CommandAction;

use std::thread;
use std::io;

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::FollowCommand(cmd) => {
            touch(command);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            touch(command);
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => touch(command)
    }
}

pub fn touch(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }

    let filename = command.args[0].replace("~", &crate::utils::home_dir());

    write(filename, "").unwrap();
}