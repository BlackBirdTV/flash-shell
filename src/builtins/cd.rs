use std::io::{Stdout};
use std::path::Path;
use std::env::{set_current_dir};
use std::thread;
use std::io;

use crate::parser::CommandAction;

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::FollowCommand(cmd) => {
            cd(command);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            cd(command);
            let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => cd(command)
    }
}

fn cd(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m\n",
            command.args.len()
        );
        return;
    }

    let path = command.args[0].replace("~", &crate::utils::home_dir());
    let path = Path::new(&path);

    if !path.exists() {
        println!("\x1b[31mDirectory \"{}\" does not exist.\x1b[0m\n",
            command.args[0].clone()
        );
        return;
    }

    set_current_dir(&path).expect(&format!("Unable to cd into {}\n", path.display()));
}