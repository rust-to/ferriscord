// The module that handles our connection to Discord's BOT web api

trait DiscordBot {
    /// The !help command
    fn help() {
        todo!();
    }
}

pub struct Connection {
    command_prefix: char,
}

impl Connection {
    pub fn new(bot_token: &str) -> Self {
        todo!();
    }

    pub fn with_prefix(mut self, prefix: &char) -> Self {

    }

    pub fn with_custom_help(mut self, f: &Fn(Context) -> i32) -> Self {

    }

    pub fn with_commands(self, prefix: char) -> Self {

    }

    pub fn with_events(self, prefix: char) -> Self {

    }
}