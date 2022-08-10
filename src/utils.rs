use whoami::username;

pub fn home_dir() -> String {
    return match crate::PLATFORM {
        "unix" => format!("/home/{}", username()),
        "win32" => format!("C:/Users/{}", username()),
        _ => panic!("Unknown Platform!")
    }
}