mod parser;
mod builtins;
mod colors;
mod utils;
mod actions;

use std::collections::HashMap;
use std::io::{self, Write, Stdout};
use std::env::{current_dir, current_exe};
use std::thread;
use crossterm::event::{Event, KeyModifiers, read, KeyEvent, KeyCode};
use parser::{parse, CommandAction};
use rlua::{Lua, Table};

#[macro_use]
extern crate crossterm;

#[macro_use]
extern crate lazy_static;

use crossterm::cursor;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use std::sync::Mutex;

pub enum Variable {
    Num(f32),
    Str(String),
    Bool(bool)
}

impl Variable {
    pub fn to_string(&self) -> String {
        match self {
            Variable::Num(value) => value.to_string(),
            Variable::Str(string) => string.to_owned(),
            Variable::Bool(value) => value.to_string(),
        }
    }
}

static mut HISTORY: Vec<parser::Command> = Vec::new();

lazy_static! {
    static ref VARIABLES: Mutex<HashMap<String, Variable>> = Mutex::new(HashMap::new());
}

pub const PLATFORM: &str = "unix";
pub static mut RED: &str = "";
pub static mut GREEN: &str = "";
pub static mut BLUE: &str = "";
pub static mut RESET: &str = "";
pub static mut BOLD: &str = "";

fn main() {
    VARIABLES.lock().unwrap().insert("1".to_owned(), Variable::Str("Hello, world!".to_owned()));
    // Enable ANSI Support for the old Windows Shell. If it fails, disable ANSI Colors.
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => unsafe {
            RED = colors::RED;
            GREEN = colors::GREEN;
            BLUE = colors::BLUE;
            BOLD = colors::BOLD;
            RESET = colors::RESET;
        }
        Err(_) => unsafe {
            RED = "";
            GREEN = "";
            BLUE = "";
            BOLD = "";
            RESET = "";
            println!("This OS doesn't support ANSI Escape Codes. Be aware, that this might lead to inconveniences.")
        }
    }

    let mut stdout = io::stdout();
    enable_raw_mode().unwrap();

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))
    .unwrap();

    // Set up STDIN
    let mut buffer = String::new();

    let mut history_idx: usize = 0;

    loop { unsafe {
        let user = whoami::username();
        let pc_name = whoami::hostname();
        let path = current_dir().expect("Invalid Path");

        // Read input from user
        let prompt = format!("{BOLD}{GREEN}{user}@{pc_name}{RESET}:{BOLD}{BLUE}{path}{RESET}↯ ", path = path.clone().into_os_string().to_str().unwrap_or("?").replace(&utils::home_dir(), "~"));

        let mut i = 0;
        let mut i_utf8 = 0usize;

        print!("{}", prompt);
        flush();

        loop {
            let key = read().unwrap();
            
            match key {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT
                }) => {
                    buffer.insert(i_utf8, c);
                    let mut offset = 0;
                    for t in buffer[i_utf8..].chars() { 
                        execute!(stdout, Print(t)).expect("Stdout error");
                        offset += t.len_utf8();
                    }
                    if buffer.len() - i > 0 { execute!(stdout, cursor::MoveLeft(offset as u16), cursor::MoveRight(c.len_utf8() as u16)).expect("Stdout error"); }
                    i += 1;
                    i_utf8 += c.len_utf8();
                    continue;
                },
                Event::Key(KeyEvent {  
                    code: KeyCode::Backspace, 
                    modifiers: KeyModifiers::NONE 
                }) => if i > 0 && buffer.len() > 0 {
                    execute!(stdout, cursor::MoveLeft(1)).expect("Stdout error");
                    i-=1;
                    i_utf8 -= buffer.chars().nth(i).unwrap().len_utf8();

                    let mut chars = buffer.chars().collect::<Vec<_>>();
                    chars.remove(i);
                    buffer = chars.clone().into_iter().collect();


                    execute!(stdout, cursor::SavePosition, Print(chars[i..].into_iter().collect::<String>()), Print(" "), cursor::RestorePosition).expect("Stdout error");
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE
                }) => {
                    break;
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL
                }) => {
                    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                    disable_raw_mode().unwrap();
                    return;
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE
                }) => if history_idx > 0 {
                    history_idx-=1;
                    if buffer.len() > 0 {
                        execute!(stdout, cursor::MoveLeft((buffer.len()) as u16)).unwrap();
                        for _ in 0..buffer.len() {execute!(stdout, Print(" ")).unwrap();}
                        execute!(stdout,  cursor::MoveLeft(buffer.len() as u16)).unwrap();
                    }
                    buffer = HISTORY[history_idx].full.clone();
                    execute!(stdout, Print(&buffer)).unwrap();
                    i = buffer.len();
                    let mut i_utf8_buf = 0;
                    for c in buffer.chars() {
                        i_utf8_buf += c.len_utf8();
                    }
                    i_utf8 = i_utf8_buf;
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE
                }) => if history_idx + 1 < HISTORY.len() {
                    history_idx+=1;
                    if buffer.len() > 0 {
                        execute!(stdout, cursor::MoveLeft((buffer.len()) as u16)).unwrap();
                        for _ in 0..buffer.len() {execute!(stdout, Print(r#" "#)).unwrap();}
                        execute!(stdout,  cursor::MoveLeft(buffer.len() as u16)).unwrap();
                    }
                    buffer = HISTORY[history_idx].full.clone();
                    execute!(stdout, Print(&buffer)).unwrap();
                    i = buffer.len();
                },
                Event::Key(KeyEvent { 
                    code: KeyCode::Left, 
                    modifiers: KeyModifiers::NONE
                }) => if i > 0 {
                    execute!(stdout, cursor::MoveLeft(1)).expect("Stdout error");
                    i -= 1;
                    i_utf8 -= buffer.chars().nth(i).unwrap().len_utf8();
                },
                Event::Key(KeyEvent { 
                    code: KeyCode::Right, 
                    modifiers: KeyModifiers::NONE
                }) => if i < buffer.chars().collect::<Vec<char>>().len() {
                    execute!(stdout, cursor::MoveRight(1)).expect("Stdout error");
                    i_utf8 += buffer.chars().nth(i).unwrap().len_utf8();
                    i += 1;
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE
                }) => {
                    if buffer.len() > 0 {
                        if i > 0 { execute!(stdout, cursor::MoveLeft(i as u16)).unwrap(); }
                        for _ in 0..buffer.len() {execute!(stdout, Print(" ")).unwrap();}
                        execute!(stdout,  cursor::MoveLeft((buffer.len() - i) as u16)).unwrap();
                    }
                    buffer = "".to_owned();
                    i = 0;
                    execute!(stdout, Print(&buffer)).unwrap();
                },
                _ => ()
            }
        }

        println!("\r");

        if buffer.len() <= 0 {
            continue;
        }

        if exec(buffer.clone(), Vec::new(), &mut stdout) { break }
        history_idx = HISTORY.len();

        // Flush buffer
        buffer = String::new();
        execute!(stdout, cursor::MoveLeft(cursor::position().unwrap_or((0, 0)).0)).unwrap();
    } }

    unsafe { print!("{RESET}"); }
    disable_raw_mode().unwrap();
    flush();
}

fn flush() {
    io::stdout().flush().unwrap()
}

fn exec(inp: String, args: Vec<String>, stdout: &mut io::Stdout) -> bool {
    let mut command = match parse(inp) {
        Some(res) => res,
        _ => return false
    };
    if args.len() > 0 {
        command.args = args;
    }

    run_command(command, stdout)
}

fn run_command(command: parser::Command, stdout: &mut Stdout) -> bool {
    match command.clone().action.as_str() {
        "echo" => builtins::ECHO(command.clone(), stdout),
        "cd" => builtins::CD(command.clone(), stdout),
        "cp" => builtins::CP(command.clone(), stdout),
        "mkdir" => builtins::MKDIR(command.clone(), stdout),
        "touch" => builtins::TOUCH(command.clone(), stdout),
        "mv" => builtins::MV(command.clone(), stdout),
        "clear" => builtins::CLEAR(command.clone(), stdout),
        "ls"=> builtins::LS(command.clone(), stdout),
        "info" => unsafe { builtins::INFO(command.clone(), stdout) },
        "pwd" => builtins::PWD(command.clone(), stdout),
        "rm" => builtins::RM(command.clone(), stdout),
        "less" => builtins::LESS(command.clone(), stdout),
        "head" => builtins::HEAD(command.clone(), stdout),
        "tail" => builtins::TAIL(command.clone(), stdout),
        "var" => builtins::VAR(command.clone(), stdout, &mut VARIABLES.lock().unwrap()),
        "history" => match command.followed_action.clone() {
            CommandAction::PipeFile(filename) => {
                let mut history: Vec<String> = Vec::new();
                for o in unsafe{&HISTORY}.to_owned() {
                    history.push(format!("{}\n", o.full));
                }
                std::fs::write(filename, utils::combine(history, " ")).expect("Unable to write to file");
            }
            CommandAction::PipeCommand(cmd) => {
                let mut history: Vec<String> = Vec::new();
                for o in unsafe{&HISTORY}.to_owned() {
                    history.push(format!("{}\n", o.full));
                }
                let mut cmd = *cmd;
                cmd.args = history;
                run_command(cmd, stdout);
            }
            CommandAction::ParallelCommand(cmd) => {
                let thread = thread::spawn(|| {
                    run_command(*cmd, &mut io::stdout());
                });
                for o in unsafe{&HISTORY}.to_owned() {
                    println!("{}\r", o.full);
                }
                thread.join().unwrap();
            }
            CommandAction::FollowCommand(cmd) => {
                for o in unsafe{&HISTORY}.to_owned() {
                    println!("{}\r", o.full);
                }
                run_command(*cmd, &mut io::stdout());
            }
            _ => for o in unsafe{&HISTORY}.to_owned() {
                println!("{}\r", o.full);
            }
        }
        "exit" => {return true}
        _ => {

            // See if an extension with the name exists
            let current_exe = current_exe().unwrap().display().to_string();
            let current_exe = current_exe[..current_exe.len()-current_exe.chars().rev().position(|c| c == '/').unwrap_or(0)].to_string();
            if std::path::Path::new(&format!("{}/exts/{}/main.lua", current_exe, command.action)).exists() {

                // If it does, run it with Lua

                let lua = Lua::new();
                let command = command.clone();
                
                lua.context(|lua_ctx| {
                    let globals = lua_ctx.globals();
                    
                    globals.set("command", command_to_table(lua_ctx, command.clone())).expect("Unable to setup Lua binding");
                    
                    let run_command = lua_ctx.create_function(|_, command: Table| {
                        
                        run_command(table_to_command(command), &mut io::stdout());
                        
                        Ok(())
                    }).unwrap();


                    globals.set("runCommand", run_command).expect("Unable to setup Lua binding");


                    lua_ctx
                        .load(
                            &std::fs::read_to_string(&format!("{}/exts/{}/main.lua", current_exe, command.action)).unwrap_or(String::new())
                        )
                        .set_name("command").unwrap()
                        .exec()
                        .expect("Error running command");
                });
            }
            else {
                let cmd = std::process::Command::new(command.action.clone())
                .args(&command.full.split(' ').collect::<Vec<&str>>()[1..])
                .spawn();
                let mut join;
                match cmd
                {
                    Ok(child) => { join = child; },
                    Err(err) => unsafe{println!("{RED}Error while running \"{}\": {}\r", command.action.as_str(), err); return false; }
                }
                join.wait().expect("Error while running extern program");
            }
        }
    }

    unsafe{HISTORY.push(command);}
    false
}

fn command_to_table(lua_ctx: rlua::Context, command: parser::Command) -> rlua::Table {
    let command_table = lua_ctx.create_table().unwrap();
    let args_table = lua_ctx.create_table().unwrap();
    for (i, arg) in command.args.iter().enumerate() {args_table.set(i+1, arg.to_owned()).expect("Unable to setup Lua binding");}
    let flags_table = lua_ctx.create_table().unwrap();
    for (i, flag) in command.args.iter().enumerate() {flags_table.set(i+1, flag.to_owned()).expect("Unable to setup Lua binding");}
    let followed_action_table = lua_ctx.create_table().unwrap();
    followed_action_table.set(1, match command.followed_action {
        CommandAction::PipeFile(_) => "PipeFile",
        CommandAction::PipeCommand(_) => "PipeCommand",
        CommandAction::FollowCommand(_) => "FollowCommand",
        CommandAction::ParallelCommand(_) => "ParallelCommand",
        _ => "None"
    }).expect("Unable to setup Lua binding");

    match command.followed_action {
        CommandAction::PipeFile(path) => followed_action_table.set(2, path).expect("Unable to setup Lua binding"),
        CommandAction::PipeCommand(cmd) => followed_action_table.set(2, command_to_table(lua_ctx, *cmd)).expect("Unable to setup Lua binding"),
        CommandAction::FollowCommand(cmd) => followed_action_table.set(2, command_to_table(lua_ctx, *cmd)).expect("Unable to setup Lua binding"),
        CommandAction::ParallelCommand(cmd) => followed_action_table.set(2, command_to_table(lua_ctx, *cmd)).expect("Unable to setup Lua binding"),
        _ => followed_action_table.set(2, rlua::Nil).expect("Unable to setup Lua binding")
    }

    command_table.set("action", command.action).expect("Unable to setup Lua binding");
    command_table.set("full", command.full).expect("Unable to setup Lua binding");
    command_table.set("args", args_table).expect("Unable to setup Lua binding");
    command_table.set("flags", flags_table).expect("Unable to setup Lua binding");
    command_table.set("followedAction", followed_action_table).expect("Unable to setup Lua binding");

    command_table
}

fn table_to_command(command: Table) -> parser::Command {
    let followed_action: rlua::prelude::LuaTable<> = command.get("followedAction").unwrap();
    parser::Command {
        action: command.get("action").unwrap_or(String::new()),
        full: command.get("full").unwrap_or(String::new()),
        args: command.get("args").unwrap_or(Vec::new()),
        flags: command.get("flags").unwrap_or(Vec::new()),
        followed_action: match followed_action.get(1).unwrap_or(String::new()).as_str() {
            "PipeFile" => CommandAction::PipeFile(
                followed_action.get(2).unwrap_or(String::new())
            ),
            "PipeCommand" => CommandAction::PipeCommand(Box::new(table_to_command(followed_action.get(2).unwrap()))),
            "FollowCommand" => CommandAction::FollowCommand(Box::new(table_to_command(followed_action.get(2).unwrap()))),
            "ParallelCommand" => CommandAction::ParallelCommand(Box::new(table_to_command(followed_action.get(2).unwrap()))),
            _ => CommandAction::NONE
        }
    }
}