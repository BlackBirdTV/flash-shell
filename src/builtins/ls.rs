use std::{fs::{read_dir}, io::Stdout, time::Duration};
use std::thread;
use std::io;

use crate::parser::{CommandAction, Command};

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) {
    match command.followed_action.clone() {
        CommandAction::PipeFile(filename) => {          
            let dir = get_dir(command.clone()).unwrap();

            let mut files: String = String::new();

            for path in dir {
                let path = path.unwrap().path();
                let name = path.display().to_string();
                files.push_str(&format!("{}{}\r\n", if command.args.len() < 1 { name[2..].to_string() } else { name }, if path.is_dir() { "/" } else { "" }));
            }

            std::fs::write(filename, files).expect("Error while writing file.");
        },
        CommandAction::PipeCommand(cmd) => {
            let mut cmd = *cmd;
            
            let dir = get_dir(command.clone()).unwrap();

            let mut files: Vec<String> = vec![];

            for path in dir {
                let path = path.unwrap().path();
                let name = path.display().to_string();
                files.push(format!("{}{}\r\n", if command.args.len() < 1 { name[2..].to_string() } else { name }, if path.is_dir() { "/" } else { "" }));
            }

            cmd.args = files;

            crate::run_command(cmd, stdout);
        },
        CommandAction::FollowCommand(cmd) => {
            ls(command);
            crate::run_command(*cmd, stdout);

        },
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            ls(command);
            let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap();
        }
        _ => {
            ls(command)
        }
    }
}

fn ls(command: crate::parser::Command) {
    let dir = get_dir(command.clone()).unwrap();
    for path in dir {
        let path = path.unwrap().path();
        let name = path.display().to_string();
        println!("{}{}\r", if command.args.len() < 1 { name[2..].to_string() } else { name }, if path.is_dir() { "/" } else { "" })
    }
}

fn get_dir(command: Command) -> Result<std::fs::ReadDir, ()> {
    if command.args.len() < 1 {
        Ok(read_dir("./").expect("Unable to read current directories contents"))
    }
    else {
        let filename = command.args[0].replace("~", &crate::utils::home_dir());
        let path = std::path::Path::new(&filename);

        if !path.exists() {
            println!("\x1b[31mDirectory \"{}\" does not exist.\x1b[0m\r",
                command.args[0].clone()
            );
            return Err(());
        }

        Ok(read_dir(filename).expect("Unable to read directories contents"))
    }
}