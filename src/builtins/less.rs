use std::io::Stdout;
use std::thread;
use std::io;

use crate::{parser::CommandAction, utils};

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::PipeFile(filename) => {
            let lines: Vec<String>;

            match get_less(command) {
                Ok(vec) => lines = vec,
                _ => {
                    return;
                }
            }

            std::fs::write(filename, utils::combine(lines, "\n")).expect("Error occurred while writing to file");
        }
        CommandAction::PipeCommand(cmd) => {
            let mut cmd = *cmd;

            let lines: Vec<String>;

            match get_less(command) {
                Ok(vec) => lines = vec,
                _ => {
                    return;
                }
            }

            cmd.args = lines;

            crate::run_command(cmd, stdout);
        }
        CommandAction::FollowCommand(cmd) => {
            less(command);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            less(command);
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => less(command)
    }
}

fn less(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }

    

    for line in get_less(command).unwrap_or(vec![]) { println!("{}\r", line); }
}

fn get_less(command: crate::parser::Command) -> Result<Vec<String>, ()> {
    let path = command.args[0].replace("~", &crate::utils::home_dir());
    let path = std::path::Path::new(&path);
    
    if !path.exists() {
        println!("\x1b[31mFile \"{}\" doesn't exist.\x1b[0m\r",
            path.file_name().unwrap().to_str().unwrap()
        );
        return Err(());
    }

    if path.is_dir() {
        println!("\x1b[31mObject \"{}\" is a directory.\x1b[0m\r",
            path.file_name().unwrap().to_str().unwrap()
        );
        return Err(());
    }

    let mut outp: Vec<String> = Vec::new();

    for item in std::fs::read_to_string(path).unwrap_or(String::new()).lines().into_iter().collect::<Vec<&str>>() {
        outp.push(item.to_owned());
    }

    Ok(outp)
}