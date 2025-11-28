use crate::irc::responses::ErrorResponse;

use super::{IRCServer, responses::CmdResponse};

#[derive(Debug)]
pub enum Command {
    PASS(String),
    NICK(String),
    USER(String, String, String, String),
    QUIT(Option<String>),
    OPER(String, String),
    SQUIT(String, String),
}

pub fn parse_command(message: &str) -> Result<(String, Command), ErrorResponse> {
    let mut prefix = String::from("");
    let string: Vec<&str> = if message.starts_with(":") {
        let initial_vec: Vec<&str> = message.split(" ").collect();
        prefix = initial_vec[0].strip_prefix(":").unwrap().into();
        initial_vec[1..].to_vec()
    } else {
        message.split(" ").collect()
    };

    let cmd = string[0];

    let command = match cmd {
        "NICK" if string.len() < 2 => Err(ErrorResponse::NeedMoreParams),
        "NICK" => Ok(Command::NICK(String::from(string[1]))),
        "PASS" if string.len() < 2 => Err(ErrorResponse::NeedMoreParams),
        "PASS" => Ok(Command::PASS(String::from(string[1]))),
        "USER" if string.len() < 4 => Err(ErrorResponse::NeedMoreParams),
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
        "QUIT" if string.len() < 2 => Ok(Command::QUIT(None)),
        "QUIT" => {
            let quit_msg = string[1..].join(" ");
            Ok(Command::QUIT(Some(quit_msg)))
        }
        _ => Err(ErrorResponse::UnknownCommand),
    };

    match command {
        Ok(cmd) => Ok((prefix, cmd)),
        Err(e) => Err(e),
    }
}

pub fn execute_command(irc: &mut IRCServer, cmd: Command) -> Result<CmdResponse, ErrorResponse> {
    match cmd {
        Command::QUIT(quit_msg) => {
            println!("User left the server\n{quit_msg:?}");
            Ok(CmdResponse::None)
        }
        _ => Ok(CmdResponse::None),
    }
}
