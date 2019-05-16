// Data types that represent things related to discord
// Example: User, Guild (server), channel, message, Role, etc

use crate::connection::CallbackContext;
use core::borrow::Borrow;

/// The command and args from a users message starting with <prefix> e.x. !ping
/// used for command callbacks
pub struct Command<'a> {
    args: [&'a str],
}

pub struct CommandIter<'a> {
    index: usize,
    data: &'a [Command<'a>],
}

impl<'a> CommandIter<'a> {
    fn new(from_data: &[Command]) -> CommandIter<'a> {
        CommandIter { index: 0,
                      data: from_data }
    }
}

impl<'a> Iterator for CommandIter<'a> {
    type Item = Command<'a>;
    fn next(&mut self) -> Option<&Command> {
        // Check to see if we've finished counting or not.
        let next_item = if self.index < self.data.len() {
            Some(&self.data[self.index])
        } else {
            None
        };

        self.count += 1;
        next_item
    }
}

/// Represents a text chat room in a discord server or private message
pub struct Channel<'a> {
    uuid: u32,
    name: &'a str,
}

impl<'a> Channel<'a> {
    pub async fn send(&self, message: &str) {
        // send a message to this channel through the discord api
    }
}

/// A Guild is discord's name for servers
pub struct Guild<'a> {
    uuid: u32,
    name: &'a str,
}

/// The user that sent the message or command
pub struct User<'a> {
    uuid: u32,
    name: &'a str,
}

/// The contents of a message
pub struct Message<'a> {
    sender: User<'a>,
    content: &'a str,
    server: Guild<'a>,
    channel: Channel<'a>,
}

/// The context data for the help command
pub struct HelpContext<T: ?Sized + CallbackContext> {
    bot_commands: [T],
}

impl<'a> HelpContext<Command<'a>> {
    fn commands(&self) -> CommandIter {
        CommandIter::new(&self.bot_commands)
    }
}
