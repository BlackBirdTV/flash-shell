use crate::utils;
use crate::actions::exec_action;

pub enum CommandAction {
    PipeFile(String),
    PipeCommand(Box<Command>),
    FollowCommand(Box<Command>),
    ParallelCommand(Box<Command>),
    NONE
}

impl Clone for CommandAction {
    fn clone(&self) -> Self {
        match self {
            CommandAction::PipeFile(path) => CommandAction::PipeFile(path.clone()),
            CommandAction::PipeCommand(command) => CommandAction::PipeCommand(Box::new((**command).clone())),
            CommandAction::FollowCommand(command) => CommandAction::FollowCommand(Box::new((**command).clone())),
            CommandAction::ParallelCommand(command) => CommandAction::ParallelCommand(Box::new((**command).clone())),
            _ => CommandAction::NONE
        }
    }
}

pub fn parse(inp: String) -> Option<Command> {
    let mut outp = Command {
        action: String::new(),
        args: vec![],
        flags: vec![],
        followed_action:  CommandAction::NONE,
        full: inp.clone(),
    };

    let mut buf = String::new();

    let mut in_str = false;
    let mut in_action = false;
    let mut pipe_file = false;
    let mut pipe_command = String::new();
    let mut parallel_command = String::new();
    let mut follow_command = String::new();

    for (i, c) in inp.chars().enumerate() {
        let any = pipe_file || in_str || in_action;
        if c == '"' {
            in_str = !in_str;
            if !in_str {
                if buf.starts_with("-") && buf.len() == 2 {
                    outp.flags.push(buf[1..].to_owned());
                }
                else if buf.starts_with("$") && crate::VARIABLES.lock().unwrap().contains_key(&buf[1..].to_owned()) {
                    outp.args.push(crate::VARIABLES.lock().unwrap().get(&buf[1..].to_owned()).unwrap().to_string())
                }
                else if buf.replace(" ", "").len() > 0 {
                    outp.args.push(buf.clone());
                }
                buf = String::new();
            }
        }
        else if c == '%' {
            in_action = !in_action;
            if !in_action {
                let action = buf.split(":").collect::<Vec<_>>();
                outp.args.push(match exec_action(action[0].to_owned(), action[1].to_owned()) {
                    Some(res) => res,
                    _ => return None
                });
                buf = String::new();
            }
        }
        else if !any && c == ' ' {
            if buf.starts_with("-") && buf.len() == 2 {
                outp.flags.push(buf[1..].to_owned());
            }
            else if buf.starts_with("$") && crate::VARIABLES.lock().unwrap().contains_key(&buf[1..].to_owned()) {
                outp.args.push(crate::VARIABLES.lock().unwrap().get(&buf[1..].to_owned()).unwrap().to_string())
            }
            else if buf.replace(" ", "").len() > 0 {
                outp.args.push(buf.clone());
            }
            buf = String::new();
        }
        else if !any && c == '>' {
            pipe_file = true;
            if buf.starts_with("-") && buf.len() == 2 {
                outp.flags.push(buf[1..].to_owned());
            }
            else if buf.starts_with("$") && crate::VARIABLES.lock().unwrap().contains_key(&buf[1..].to_owned()) {
                outp.args.push(crate::VARIABLES.lock().unwrap().get(&buf[1..].to_owned()).unwrap().to_string())
            }
            else if buf.replace(" ", "").len() > 0 {
                outp.args.push(buf.clone());
            }
            buf = String::new();
        }
        else if !any && c == '|' {
            pipe_command = inp[i+1..].to_string();
            break;
        }
        else if !any && c == '&' {
            follow_command = inp[i+1..].to_string();
            break;
        }
        else if !any && c == '~' {
            parallel_command = inp[i+1..].to_string();
            break;
        }
        else {
            buf.push((&c).to_owned());
        }
    }
    if pipe_file {
        outp.followed_action = CommandAction::PipeFile(utils::trim(buf.clone()));
    }
    else if pipe_command.len() > 0 {
        outp.followed_action = CommandAction::PipeCommand(Box::new(match parse(utils::trim(pipe_command.clone())) {
            Some(res) => res,
            _ => return None
        }));
    }
    else if follow_command.len() > 0 {
        outp.followed_action = CommandAction::FollowCommand(Box::new(match parse(utils::trim(follow_command.clone())) {
            Some(res) => res,
            _ => return None
        }));
    }
    else if parallel_command.len() > 0 {
        outp.followed_action = CommandAction::ParallelCommand(Box::new(match parse(utils::trim(parallel_command.clone())) {
            Some(res) => res,
            _ => return None
        }));
    }
    else if buf.starts_with("-") && buf.len() == 2 {
        outp.flags.push(buf[1..].to_owned());
    }
    else if buf.starts_with("$") && crate::VARIABLES.lock().unwrap().contains_key(&buf[1..].to_owned()) {
        outp.args.push(crate::VARIABLES.lock().unwrap().get(&buf[1..].to_owned()).unwrap().to_string())
    }
    else if buf.replace(" ", "").len() > 0 {
        outp.args.push(buf.clone());
    }

    if outp.args.len() > 0 {
        outp.action = outp.args[0].clone();
        outp.args = outp.args[1..].to_vec();
    }

    Some(outp)
}

pub struct Command {
    pub action: String,
    pub args: Vec<String>,
    pub flags: Vec<String>,
    pub followed_action: CommandAction,
    pub full: String
}

impl Command {
    pub fn contains_flag(&self, flag: &str) -> bool {
        for o in &self.flags {
            if o == flag {
                return true;
            }
        }
        false
    }

    pub fn clone(&self) -> Command {
        Command {
            action: self.action.clone(),
            args: self.args.clone(),
            flags: self.flags.clone(),
            followed_action: self.followed_action.clone(),
            full: self.full.clone(),
        }
    }
}