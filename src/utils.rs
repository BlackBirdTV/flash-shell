use whoami::username;

pub fn home_dir() -> String {
    return match crate::PLATFORM {
        "unix" => format!("/home/{}", username()),
        "win32" => format!("C:/Users/{}", username()),
        _ => panic!("Unknown Platform!")
    }
}

pub fn combine(array: Vec<String>, sequence: &str) -> String {
    let mut outp = String::new();

    for item in array {
        outp = format!("{}{}{}", outp, item, sequence);
    }

    outp
}

pub fn trim(inp: String) -> String {
    let mut outp = inp;
    while outp.starts_with(" ") {        
        outp = outp[1..].to_string();
    }

    while outp.ends_with(" ") {        
        outp = outp[..outp.len()-1].to_string();
    }

    outp
}

/*

---------- PLANNED FOR THUNDER ----------

pub fn truncate(inp: String, n: usize) -> String {
    if inp.len() == n { return inp; }

    let mut outp;

    if inp.len() > n {
        outp = format!("{}...", inp[..n-3].to_owned());
    }
    else {
        outp = inp;
        while outp.len() < n {
            outp.push(' ');
        }
    }

    outp
}

pub fn center(text: String, filler: char, width: usize) -> String {
    let len = text.len();
    let filler_amount: usize = (width - len) / 2;
    format!("{}{}{}", repeat(filler.to_string(), filler_amount + if (width % 2 == 0 && len % 2 != 0) || (width % 2 != 0  && len % 2 == 0)  {1} else {0}), text, repeat(filler.to_string(), filler_amount))
}

pub fn repeat(sequence: String, n: usize) -> String {
    let mut outp = String::new();

    for _ in 0..n {
        outp.push_str(&sequence);
    }

    outp
}
*/