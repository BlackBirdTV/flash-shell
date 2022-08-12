use std::{io::Stdout};
use std::thread;
use std::io;

use crate::parser::CommandAction;

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::FollowCommand(cmd) => {
            rm(command);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            rm(command);
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => rm(command)
    }
}

pub fn rm(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }

    let path = command.args[0].replace("~", &crate::utils::home_dir());
    let path = std::path::Path::new(&path);

    if !path.exists() {
        println!("\x1b[31mFile or Directory \"{}\" does not exist.\x1b[0m\r",
            command.args[0].clone()
        );
        return;
    }

    let is_dir = path.is_dir();
    let empty = is_dir && std::fs::read_dir(path).unwrap().count() == 0;
    let recursive = command.contains_flag("r");

    if recursive && is_dir {
        std::fs::remove_dir_all(path).unwrap();
    }
    else if empty {
        std::fs::remove_dir(path).unwrap();
    }
    else if !is_dir {
        std::fs::remove_file(path).unwrap();
    }
    else if !empty && is_dir {
        println!("\x1b[31mDirectory \"{}\" is not empty. Add the \"-r\" flag to delete all of its contents.\x1b[0m\r",
            command.args[0].clone()
        );
        return;
    }
}