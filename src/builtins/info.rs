use std::io::Stdout;
use std::thread;
use std::io;

use crate::{RESET, BOLD, parser::CommandAction};

pub fn main(command: crate::parser::Command, stdout: &mut Stdout) { 
    match command.followed_action.clone() {
        CommandAction::PipeFile(filename) => {
            std::fs::write(filename, get_info()).expect("Error occurred while writing to file");
        }
        CommandAction::PipeCommand(cmd) => {
            let mut cmd = *cmd;

            cmd.args = vec![get_info()];

            crate::run_command(cmd, stdout);
        }
        CommandAction::FollowCommand(cmd) => {
            info(command);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            // TODO
            info(command);
    let thread1 = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            thread1.join().unwrap()
        }
        _ => info(command)
    }
}

fn info(command: crate::parser::Command) {
    if command.args.len() != 0 {
        println!("\x1b[31mExpected 0 arguments but received {}\x1b[0m",
            command.args.len()
        );
        return;
      }
      println!("{}", get_info());
}

fn get_info() -> String {
   unsafe { format!("
                  ..              {BOLD}╔══════════╣INFO╠══════════╗{RESET}\r
                ...               {BOLD}║{RESET}                          {BOLD}║{RESET}\r
              .....               {BOLD}║{RESET}        FLASH SHELL       {BOLD}║{RESET}\r
            ......                {BOLD}║{RESET}                          {BOLD}║{RESET}\r
          .......                 {BOLD}║{RESET} Version         Beta 1.0 {BOLD}║{RESET}\r
         ........                 {BOLD}║{RESET}                          {BOLD}║{RESET}\r
       ...................        {BOLD}║{RESET}                          {BOLD}║{RESET}\r
     ...................          {BOLD}║{RESET}                          {BOLD}║{RESET}\r
   ...................            {BOLD}║{RESET}                          {BOLD}║{RESET}\r
            ........              {BOLD}║{RESET}                          {BOLD}║{RESET}\r
           .......                {BOLD}║{RESET}                          {BOLD}║{RESET}\r
           ......                 {BOLD}║{RESET}                          {BOLD}║{RESET}\r
          .....                   {BOLD}║{RESET}                          {BOLD}║{RESET}\r
         ....                     {BOLD}║{RESET}                          {BOLD}║{RESET}\r
        ..                        {BOLD}╚══════════════════════════╝{RESET}\r
  ")
}
}