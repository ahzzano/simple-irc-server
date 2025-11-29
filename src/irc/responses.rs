#[derive(Debug)]
pub enum Response {
    Error(ErrorResponse),
    Command(),
}

#[derive(Debug)]
pub enum ErrorResponse {
    NoSuchNick = 401,
    NoSuchServer = 402,
    UnknownCommand = 421,
    NeedMoreParams = 461,
    AlreadyRegistered = 462,
}

#[derive(Debug)]
pub enum CmdResponse {
    None,
    Userhost,
}
