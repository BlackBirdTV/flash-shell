pub fn parse(inp: String) -> Command {
    let mut outp = Command {
        action: String::new(),
        args: vec![],
        flags: vec![],
        full: inp.clone()
    };

    let mut buf = String::new();

    let mut in_str = false;

    for c in inp.chars() {
        if c == '"' {
            in_str = !in_str;
            if !in_str {
                if buf.starts_with("-") && buf.len() == 2 {
                    outp.flags.push(buf[1..].to_owned());
                }
                else {
                    outp.args.push(buf.clone());
                }
                buf = String::new();
            }
        }
        else if !in_str && c == ' ' {
            if buf.starts_with("-") && buf.len() == 2 {
                outp.flags.push(buf[1..].to_owned());
            }
            else {
                outp.args.push(buf.clone());
            }
            buf = String::new();
        }
        else {
            buf.push((&c).to_owned());
        }
    }
    if buf.starts_with("-") && buf.len() == 2 {
        outp.flags.push(buf[1..].to_owned());
    }
    else {
        outp.args.push(buf.clone());
    }

    if outp.args.len() > 0 {
        outp.action = outp.args[0].clone();
        outp.args = outp.args[1..].to_vec();
    }

    outp
}

pub struct Command {
    pub action: String,
    pub args: Vec<String>,
    pub flags: Vec<String>,
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
            full: self.full.clone(),
        }
    }
}