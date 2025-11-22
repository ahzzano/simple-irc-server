#[derive(Debug)]
pub enum Command {
    PASS(String),
    NICK(String),
    USER(String, String, String, String),
}

pub fn parse_command(message: &String) -> Option<Command> {
    if message.starts_with(":") {
        None
    } else {
        let string: Vec<&str> = message.split(" ").collect();
        let cmd = string[0];

        return match cmd {
            "NICK" => Some(Command::NICK(String::from(string[1]))),
            "PASS" => Some(Command::PASS(String::from(string[1]))),
            _ => None,
        };
    }
}
