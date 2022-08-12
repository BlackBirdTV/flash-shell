use std::{fs, io::Stdout};
use std::thread;
use std::io;

use crossterm::style::Print;

use crate::{parser::{CommandAction, Command}, utils::combine};

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::PipeFile(path) => {
            fs::write(path, combine(command.args, " ")).expect("Unable to write file to.");
            return;
        },
        CommandAction::PipeCommand(cmd) => {
            let mut cmd = *cmd;
 
            cmd.args = command.args;

            crate::run_command(cmd, stdout);
        },
        CommandAction::FollowCommand(cmd) => {
            echo(command, stdout);
            crate::run_command(*cmd, stdout);
        },
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            echo(command, stdout);
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        },
        _ => {
            echo(command, stdout);
        }
    }
}

fn echo(command: Command, stdout: &mut Stdout) {
    let len = command.args.len();
    for (i, arg) in command.args.iter().enumerate() {
        execute!(stdout, Print(arg)).expect("Error occurred while interacting with console");
        if i < len - 1 && !arg.ends_with("\r\n") {
            print!(" ");
        }
    }
    if !command.contains_flag("c") { println!("\r"); }
}