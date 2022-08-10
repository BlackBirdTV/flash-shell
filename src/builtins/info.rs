use crate::{RESET, BOLD};

pub unsafe fn main(command: crate::parser::Command) { 
    if command.args.len() != 0 {
      println!("\x1b[31mExpected 0 arguments but received {}\x1b[0m",
          command.args.len()
      );
      return;
    }
    println!("
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
");
}