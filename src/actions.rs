use crate::utils;

use crate::colors::{RED, RESET};

use fasteval::{EmptyNamespace, ez_eval};

pub fn exec_action(action: String, inp: String) -> Option<String> {
    let mut ns = EmptyNamespace;
    match action.as_str() {
        "trim" => Some(utils::trim(inp)),
        "calc" => match ez_eval(&inp, &mut ns) {
            Ok(res) => Some(res.to_string()),
            _ => {
                println!("{RED}Invalid operation: {action}\r{RESET}");
                None
            }
        }
        _ => {
            println!("{RED}Action \"{action}\" does not exist.\r{RESET}");
            None
        }
    }
}