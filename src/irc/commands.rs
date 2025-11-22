use crate::irc::responses::ErrorResponse;

#[derive(Debug)]
pub enum Command {
    PASS(String),
    NICK(String),
    USER(String, String, String, String),
}

pub fn parse_command(message: &String) -> Result<(String, Command), ErrorResponse> {
    let mut prefix = String::from("");
    let string: Vec<&str> = if message.starts_with(":") {
        message.split(" ").collect()
    } else {
        let initial_vec: Vec<&str> = message.split(" ").collect();
        prefix = initial_vec[0].strip_prefix(":").unwrap().into();
        initial_vec[1..].to_vec()
    };

    let cmd = string[0];

    let command = match cmd {
        "NICK" => Ok(Command::NICK(String::from(string[1]))),
        "PASS" => Ok(Command::PASS(String::from(string[1]))),
        "USER" => {
            let username = String::from(string[1]);
            let hostname = String::from(string[2]);
            let servername = String::from(string[3]);
            let name = string[4..].join(" ");
            let name = name.strip_prefix(":").unwrap();
            Ok(Command::USER(
                username,
                hostname,
                servername,
                name.to_string(),
            ))
        }
        _ => Err(ErrorResponse::UnknownCommand),
    };

    match command {
        Ok(cmd) => Ok((prefix, cmd)),
        Err(e) => Err(e),
    }
}
