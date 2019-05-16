use crate::Context;
use actix::Message;

use crate::discord_datatypes::*;

/// Library users will make a struct implementing this trait
/// callback functions for Commands and Events will be registered in here
/// A proc macro will allow doing most of this automatically
trait DiscordBot {
    // setup functions
    // these get called when the bot is started
    // can be implemented automatically with a macro

    fn register_commands();

    fn register_events();

    fn register_help<'a>() -> &'a Fn(HelpContext) {
        // default help function

    }

    /// The !help command. If not implemented by user it will use an auto generated !help implementation
    fn help(ctx: HelpContext<?Sized>) {
        println!("not implemented");
    }

    /// This is called to start the connection and then call the main bot loop
    fn run(self) -> Self;

    // Callback functions:

    /// Called when the bot receives an event
    fn on_event() {}

    fn on_command() {}
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
        Connection { api_token: "",
                     command_prefix: '!' }
    }
}

impl<'a> DiscordBot for Connection<'a> {
    fn run(self) -> Self {
        // connect to the discord api with self.api_token
        self
    }
}

impl<'a> Connection<'a> {
    // Builder functions

    /// Start a new Connection instance with the supplied bot token
    pub fn new(bot_token: &'a str) -> Self {
        Connection { api_token: bot_token,
                     command_prefix: Default::default() }
    }

    /// Assign a custom prefix for the bot's commands
    pub fn with_prefix(mut self, prefix: &char) -> Self {
        self
    }

    /// For using your own help function (or disable it by returning None)
    pub fn with_custom_help(mut self, f: &Fn(Context) -> Option<String>) -> Self {
        self
    }

    pub fn with_commands(self, f: &Fn(Context)) -> Self {
        self
    }

    pub fn with_events(self, f: &Fn(Context)) -> Self {
        self
    }

    // Usage functions

    /// Sends a message to every server the user is connected to
    pub fn broadcast_message(message: &str) -> Result<(), &str> {
        Ok(())
    }
}

// Another possible Usage format:

//#[derive(DiscordBot)] //this derive will add all of the #[command] #[event] #[help] tagged functions automatically
//struct MyBot;
//
//impl MyBot {
//    #[command]
//    fn ping(ctx: Context<Command>) {}
//
//    #[help]
//    fn help(ctx: Context<Help>) {}
//
//    #[event]
//    fn on_message(ctx: Context<Event>) {}
//}
