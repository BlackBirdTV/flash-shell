mod parser;
mod builtins;
mod colors;
mod utils;

use std::io::{self, Write};
use std::env::{current_dir, current_exe};
use parser::{parse};
use rlua::Lua;

//importing in execute! macro
#[macro_use]
extern crate crossterm;

use crossterm::cursor::{self};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

static mut HISTORY: Vec<parser::Command> = Vec::new();

pub const PLATFORM: &str = "unix";
pub static mut RED: &str = "";
pub static mut GREEN: &str = "";
pub static mut BLUE: &str = "";
pub static mut RESET: &str = "";
pub static mut BOLD: &str = "";

const CHARS: &str = "aäbcdefghijklmnoöpqrstuüvwxyzAÄBCDEFGHIJKLMNOÖPQRSTUÜVWXYZ1234567890.;:_^°, -+#*'~|<>!\"§$%&/()=?`´{[]}\\";

fn main() {

    // Enable ANSI Support for the old Windows Shell. If it fails, disable ANSI Colors.
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => unsafe {
            RED = colors::RED;
            GREEN = colors::GREEN;
            BLUE = colors::BLUE;
            BOLD = colors::BOLD;
            RESET = colors::RESET;
        }
        Err(_) => {
            // TODO: Disable ANSI
            println!("This OS doesn't support ANSI Escape Codes. Be aware, that this might led to inconveniences.")
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
        let prompt_unformatted = format!("{user}@{pc_name}:{path}↯ ", path = path.clone().into_os_string().to_str().unwrap_or("?").replace(&utils::home_dir(), "~"));
        print!("{}", prompt);
        flush();

        loop {
            let key = read().unwrap();

            for c in CHARS.chars() {
                if key == Event::Key(KeyEvent { 
                    code: KeyCode::Char(c),
                    modifiers: KeyModifiers::NONE
                })
                {
                    buffer.push(c);
                    execute!(stdout, Print(c)).unwrap();
                    continue;
                }
            }

            match key {
                Event::Key(KeyEvent { 
                    code: KeyCode::Backspace, 
                    modifiers: KeyModifiers::NONE 
                }) => {
                    if buffer.len() > 0 {
                        execute!(stdout, cursor::MoveLeft(1), Print(r#" "#), cursor::MoveLeft(1)).unwrap();
                        buffer.pop();
                    }
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
                }) => if history_idx > 0{
                    history_idx-=1;
                    if buffer.len() > 0 {
                        execute!(stdout, cursor::MoveLeft((buffer.len()) as u16)).unwrap();
                        for _ in 0..buffer.len() {execute!(stdout, Print(r#" "#)).unwrap();}
                        execute!(stdout,  cursor::MoveLeft(buffer.len() as u16)).unwrap();
                    }
                    buffer = HISTORY[history_idx].full.clone();
                    execute!(stdout, Print(&buffer)).unwrap();
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
                },
                Event::Key(KeyEvent { 
                    code: KeyCode::Left, 
                    modifiers: KeyModifiers::NONE
                }) => {
                //    println!("{} : {}", cursor::position().unwrap().0, prompt.len() as u16 );
                if cursor::position().unwrap().0 > prompt_unformatted.len() as u16  - 2{
                    execute!(stdout, cursor::MoveLeft(1)).unwrap();
                }},
                Event::Key(KeyEvent { 
                    code: KeyCode::Right, 
                    modifiers: KeyModifiers::NONE
                }) => {
                //    println!("{} : {}", cursor::position().unwrap().0, prompt.len() as u16 );
                if cursor::position().unwrap().0 < prompt_unformatted.len() as u16 + buffer.len() as u16 - 2{
                    execute!(stdout, cursor::MoveRight(1)).unwrap();
                }},
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE
                }) => {
                    if buffer.len() > 0 {
                        execute!(stdout, cursor::MoveLeft((buffer.len()) as u16)).unwrap();
                        for _ in 0..buffer.len() {execute!(stdout, Print(r#" "#)).unwrap();}
                        execute!(stdout,  cursor::MoveLeft(buffer.len() as u16)).unwrap();
                    }
                    buffer = "".to_owned();
                    execute!(stdout, Print(&buffer)).unwrap();
                },
                _ => ()
            }
        }

        println!("\r");

        if buffer.len() <= 0 {
            continue;
        }

        if exec(buffer.clone(), &mut stdout) { break }
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

unsafe fn exec(inp: String, _stdout: &mut io::Stdout) -> bool {
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
        "HISTORY" => {
            for o in (&HISTORY).to_owned() {
                println!("{}\r", o.full);
            }
        },
        "exit" => {return true}
        _ => {

            // See if an extension with the name exists
            let current_exe = current_exe().unwrap().display().to_string();
            let current_exe = current_exe[..current_exe.len()-current_exe.chars().rev().position(|c| c == '/').unwrap_or(0)].to_string();
            if std::path::Path::new(&format!("{}/exts/{}/main.lua", current_exe, command.action)).exists() {

                // If it does, run it with Lua
                // TODO: Run it with real Lua bc hlua doesn't implement the standard library

                let mut lua = Lua::new();
                
                lua.context(|lua_ctx| {
                    let globals = lua_ctx.globals();
            
                    let args = lua_ctx.create_table().unwrap();
                    for (i, s) in command.args.iter().enumerate() {
                        args.set(i+1, s.to_owned().as_str()).expect("Unable to set global 'args'");
                    }
                    globals.set("args", args).unwrap();

                    let flags = lua_ctx.create_table().unwrap();
                    for (i, s) in command.flags.iter().enumerate() {
                        flags.set(i+1, s.to_owned().as_str()).expect("Unable to set global 'flags'");
                    }
                    globals.set("flags", flags).unwrap();
            
                    lua_ctx
                        .load(
                            &std::fs::read_to_string(&format!("{}/exts/{}/main.lua", current_exe, command.action)).unwrap_or(String::new())
                        )
                        .set_name("command").unwrap()
                        .exec()
                        .expect("Error running module");
                });
            }
            else {
                println!("{RED}Unknown command: {}\r", command.action.as_str());
            }
        }
    }

    HISTORY.push(command);
    false
}
