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