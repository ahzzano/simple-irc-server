#[derive(Default, Debug)]
pub struct User {
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub realname: String,
    pub registered: bool,
}

impl User {
    pub fn with_nickname(mut self, nickname: String) -> Self {
        self.nickname = nickname;
        self
    }
    pub fn with_user(mut self, username: String, realname: String) -> Self {
        self.username = username;
        self.realname = realname;
        self
    }
    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = nickname;
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_realname(&mut self, realname: String) {
        self.realname = realname;
    }
}
