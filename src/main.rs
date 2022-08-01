mod parser;
mod builtins;

use std::io::{self, Write};
use std::env::{current_dir, home_dir, current_exe};
use parser::{parse};
use hlua::Lua;

const green: &str = "\x1b[92m";
const blue: &str = "\x1b[94m";
const red: &str = "\x1b[31m";
const reset: &str = "\x1b[0m";
const bold: &str = "\x1b[1m";

static mut history: Vec<parser::Command> = Vec::new();

fn main() {

    // Enable ANSI Support for the old Windows Shell. If it fails, disable ANSI Colors.
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {}
        Err(_) => {}
    }

    // Set up STDIN
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        let user = whoami::username();
        let pc_name = whoami::hostname();
        let path = current_dir().expect("Invalid Path");

        // Read input from user
        print!("{bold}{green}{user}@{pc_name}{reset}:{bold}{blue}{path}{reset}↯ ", path = path.into_os_string().to_str().unwrap_or("?").replace(&home_dir().unwrap().display().to_string(), "~"));
        flush();
        stdin.read_line(&mut buffer).expect("Failed to read from stdin");

        if exec((&buffer[..buffer.len()-1]).to_owned()) { break }

        // Flush buffer
        buffer = String::new();
    }

    print!("{reset}");
    flush();
}

fn flush() {
    io::stdout().flush().unwrap()
}

fn exec(inp: String) -> bool {
    let command = parse(inp);

    match command.clone().action.as_str() {
        "echo" => builtins::ECHO(command.clone()),
        "cd" => builtins::CD(command.clone()),
        "cp" => builtins::CP(command.clone()),
        "mkdir" => builtins::MKDIR(command.clone()),
        "touch" => builtins::TOUCH(command.clone()),
        "mv" => builtins::MV(command.clone()),
        "clear" => builtins::CLEAR(command.clone()),
        "ls"=> builtins::LS(command.clone()),
        "info" => builtins::INFO(command.clone()),
        "pwd" => builtins::PWD(command.clone()),
        "rm" => builtins::RM(command.clone()),
        "less" => builtins::LESS(command.clone()),
        "head" => builtins::HEAD(command.clone()),
        "tail" => builtins::TAIL(command.clone()),
        "history" => {
            for o in unsafe { history.iter().rev() } {
                println!("{}", o.full);
            }
        },
        "exit" => {return true}
        _ => {
            let current_exe = current_exe().unwrap().display().to_string();
            let current_exe = current_exe[..current_exe.len()-current_exe.chars().rev().position(|c| c == '/').unwrap_or(0)].to_string();
            if std::path::Path::new(&format!("{}/exts/{}", current_exe, command.action)).exists() {
                let mut lua = Lua::new();
                lua.set("print", hlua::function1(|o: hlua::AnyLuaString| {
                    print!("{}", String::from_utf8(o.0).unwrap())
                }));
                let mut table = String::from("return {");
                for arg in command.clone().args {
                    table.push_str(&format!("\"{}\",", arg));
                }
                table.push('}');
                lua.set("argsLen", command.args.len() as u32);
                lua.checked_set("getArgs", 
                    hlua::LuaCode(&table)
                ).unwrap();
                let mut table = String::from("return {");
                for flag in command.clone().flags {
                    table.push_str(&format!("\"{}\",", flag));
                }
                table.push('}');
                lua.set("flagsLen", command.flags.len() as u32);
                lua.checked_set("getFlags", 
                    hlua::LuaCode(&table)
                ).unwrap();
                lua.execute::<()>(&std::fs::read_to_string(&format!("{}/exts/{}", current_exe, command.action)).unwrap_or(String::new())).unwrap();
            }
            else {
                println!("{red}Unknown command: {}", command.action.as_str().replace(" ", "_").replace("\n", "-"));
            }
        }
    }

    unsafe {
        history.insert(0, command);
    }

    false
}
