#[derive(Debug)]
enum Command {
    PASS(String),
    NICK(String),
}

pub fn parse_command(message: String) -> Option<Command> {
    None
}
