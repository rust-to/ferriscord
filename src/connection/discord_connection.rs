use crate::Context;

trait DiscordBot {
    /// The !help command
    fn help(ctx: Context) {
        println!("not implemented");
    }

    fn run(self) -> Self;
}

/// The Connection object handles all things related to our connection to Discord's BOT web api
/// as well as essentially being the main "Bot" object. A "discord bot" is just a connection
/// to the api with a token
pub struct Connection<'a> {
    api_token: &'a str,
    command_prefix: char,
}

impl<'a> Default for Connection<'a> {
    fn default() -> Self {
        Connection {
            api_token: "",
            command_prefix: '!',
        }
    }
}

impl<'a> DiscordBot for Connection<'a> {
    fn run(self) -> Self {
        // connect to the discord api with self.api_token
        self
    }
}

impl<'a> Connection<'a> {
    pub fn new(bot_token: &'a str) -> Self {
        Connection {
            api_token: bot_token,
            command_prefix: Default::default(),
        }
    }

    pub fn with_prefix(mut self, prefix: &char) -> Self {
        self
    }

    pub fn with_custom_help(mut self, f: &Fn(Context)) -> Self {
        self
    }

    pub fn with_commands(self, f: &Fn(Context)) -> Self {
        self
    }

    pub fn with_events(self, f: &Fn(Context)) -> Self {
        self
    }
}
