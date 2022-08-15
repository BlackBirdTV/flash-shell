use std::{io::Stdout};

mod info;
pub const INFO: unsafe fn(crate::parser::Command, &mut Stdout) -> () = info::main;

mod ls;
pub const LS: fn(crate::parser::Command, &mut Stdout) -> () = ls::main;

mod cd;
pub const CD: fn(crate::parser::Command, &mut Stdout) -> () = cd::main;

mod cp;
pub const CP: fn(crate::parser::Command, &mut Stdout) -> () = cp::main;

mod echo; 
pub const ECHO: fn(crate::parser::Command, &mut Stdout) -> () = echo::main;

mod clear;
pub const CLEAR: fn(crate::parser::Command, &mut Stdout) -> () = clear::main;

mod mkdir;
pub const MKDIR: fn(crate::parser::Command, &mut Stdout) -> () = mkdir::main;

mod touch;
pub const TOUCH: fn(crate::parser::Command, &mut Stdout) -> () = touch::main;

mod pwd;
pub const PWD: fn(crate::parser::Command, &mut Stdout) -> () = pwd::main;

mod mv;
pub const MV: fn(crate::parser::Command, &mut Stdout) -> () = mv::main;

mod rm;
pub const RM: fn(crate::parser::Command, &mut Stdout) -> () = rm::main;

mod less;
pub const LESS: fn(crate::parser::Command, &mut Stdout) -> () = less::main;

mod head;
pub const HEAD: fn(crate::parser::Command, &mut Stdout) -> () = head::main;

mod tail;
pub const TAIL: fn(crate::parser::Command, &mut Stdout) -> () = tail::main;

mod var;
pub const VAR: fn(crate::parser::Command, &mut Stdout, &mut crate::HashMap<String, crate::Variable>) -> () = var::main;