use std::{thread, io};

use crate::{parser::CommandAction, Variable, utils};

pub fn main(command: crate::parser::Command, stdout: &mut std::io::Stdout, variables: &mut crate::HashMap<String, crate::Variable>) {
    match command.followed_action.clone() {
        CommandAction::FollowCommand(cmd) => {
            var(command, variables);
            crate::run_command(*cmd, stdout);
        }
        CommandAction::ParallelCommand(cmd) => {
            let thread = thread::spawn(|| {
                crate::run_command(*cmd, &mut io::stdout());
            });
            var(command, variables);
            thread.join().unwrap();
        }
        _=> var(command, variables)
    }
}

fn var(command: crate::parser::Command, variables: &mut crate::HashMap<String, crate::Variable>) {
    if command.args.len() != 2 {
        println!("\x1b[31mExpected 2 arguments but received {}\x1b[0m\r",
            command.args.len()
        );
        return;
    }

    let value = command.args[1].as_str();
    let variable;

    if value == "true" || value == "false"  {
        variable = Variable::Bool(value == "true")
    }
    else if utils::is_numeric(value.to_owned()) {
        variable = Variable::Num(value.parse::<f32>().unwrap());
    }
    else {
        variable = Variable::Str(value.to_owned());
    }
    let args = command.args;
    variables.insert(args[0].clone(), variable);
}