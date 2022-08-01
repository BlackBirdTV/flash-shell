pub fn main(command: crate::parser::Command) {
  if command.args.len() != 0 {
    println!("\x1b[31mExpected 0 arguments but received {}\x1b[0m",
        command.args.len()
    );
    return;
  }
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";
    println!("
                ..              {bold}╔══════════╣INFO╠══════════╗{reset}
              ...               {bold}║{reset}                          {bold}║{reset}
            .....               {bold}║{reset}        FLASH SHELL       {bold}║{reset}
          ......                {bold}║{reset}                          {bold}║{reset}
        .......                 {bold}║{reset} Version         Beta 1.0 {bold}║{reset}
       ........                 {bold}║{reset}                          {bold}║{reset}
     ...................        {bold}║{reset}                          {bold}║{reset}
   ...................          {bold}║{reset}                          {bold}║{reset}
 ...................            {bold}║{reset}                          {bold}║{reset}
          ........              {bold}║{reset}                          {bold}║{reset}
         .......                {bold}║{reset}                          {bold}║{reset}
         ......                 {bold}║{reset}                          {bold}║{reset}
        .....                   {bold}║{reset}                          {bold}║{reset}
       ....                     {bold}║{reset}                          {bold}║{reset}
      ..                        {bold}╚══════════════════════════╝{reset}
");
}