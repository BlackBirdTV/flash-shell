mod info;
pub const INFO: unsafe fn(crate::parser::Command) -> () = info::main;

mod ls;
pub const LS: fn(crate::parser::Command) -> () = ls::main;

mod cd;
pub const CD: fn(crate::parser::Command) -> () = cd::main;

mod cp;
pub const CP: fn(crate::parser::Command) -> () = cp::main;

mod echo; 
pub const ECHO: fn(crate::parser::Command) -> () = echo::main;

mod clear;
pub const CLEAR: fn(crate::parser::Command) -> () = clear::main;

mod mkdir;
pub const MKDIR: fn(crate::parser::Command) -> () = mkdir::main;

mod touch;
pub const TOUCH: fn(crate::parser::Command) -> () = touch::main;

mod pwd;
pub const PWD: fn(crate::parser::Command) -> () = pwd::main;

mod mv;
pub const MV: fn(crate::parser::Command) -> () = mv::main;

mod rm;
pub const RM: fn(crate::parser::Command) -> () = rm::main;

mod less;
pub const LESS: fn(crate::parser::Command) -> () = less::main;

mod head;
pub const HEAD: fn(crate::parser::Command) -> () = head::main;

mod tail;
pub const TAIL: fn(crate::parser::Command) -> () = tail::main;