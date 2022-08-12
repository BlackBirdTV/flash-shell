use std::fs::File;
use std::io::{prelude::*, BufReader, Stdout};

use crate::parser::CommandAction;
use crate::utils;

use std::thread;
use std::io;

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::PipeFile(filename) => {
            let originfilename = command.args[0].replace("~", &crate::utils::home_dir());

            let lines: Vec<String>;

            match get_tail(originfilename) {
                Ok(vec) => lines = vec,
                _ => {
                    return;
                }
            }

            std::fs::write(filename, utils::combine(lines, "\n")).expect("Error occurred while writing to file");
        }
        CommandAction::PipeCommand(cmd) => {
            let mut cmd = *cmd;

            let filename = command.args[0].replace("~", &crate::utils::home_dir());

            let lines: Vec<String>;

            match get_tail(filename) {
                Ok(vec) => lines = vec,
                _ => {
                    return;
                }
            }

            cmd.args = lines;

            crate::run_command(cmd, stdout);
        }
        CommandAction::FollowCommand(cmd) => {
            tail(command);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            tail(command);
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => tail(command)
    }
}

pub fn tail(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }
    
    let filename = command.args[0].replace("~", &crate::utils::home_dir());

    let lines;

    match get_tail(filename) {
        Ok(vec) => lines = vec,
        _ => {
            return;
        }
    }

    for line in lines {
        println!("{}\r", line)
    }
}

fn get_tail(filename: String) -> Result<Vec<String>, ()> {
    {
        let path = std::path::Path::new(&filename);

        if !path.exists() {
            println!("\x1b[31mFile or Directory \"{}\" does not exist.\x1b[0m\r",
                filename
            );
            return Err(());
        }
    }

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    
    Ok(lines[if lines.len() >= 10 { lines.len()-10 } else { 0 }..].to_vec())
}