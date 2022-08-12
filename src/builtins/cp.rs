use std::{fs::{copy}, io::Stdout};
use std::thread;
use std::io;

use crate::parser::CommandAction;

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::FollowCommand(cmd) => {
            cp(command);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            cp(command);
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => cp(command)
    }
}

fn cp(command: crate::parser::Command) {
    if command.args.len() != 2 {
        println!("\x1b[31mExpected 2 arguments but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }

    let path0 = command.args[0].replace("~", &crate::utils::home_dir());
    let path = std::path::Path::new(&path0);

    if !path.exists() {
        println!("\x1b[31mFile or Directory \"{}\" does not exist.\x1b[0m\r",
            command.args[0].clone()
        );
        return;
    }

    let path1 = command.args[1].replace("~", &crate::utils::home_dir());

    copy(path0, path1).unwrap();
}