/// Context data gets passed into callbacks for functions and events
/// provides useful objects such as the Message{User, Content}, Command{User, args}, the channel, server
pub struct Context {
    user: User,
    message: Option<Message>,
    command: Option<Command>,
    channel: Channel,
    guild: Option<Guild>,
}

impl Context {
    fn send(&self, message: &str) {
        self.channel.send(message);
    }
}

/// The details associated with a Message
struct Message {}

/// The command and args from a users message starting with <prefix> e.x. !ping
/// used for command callbacks
struct Command {}

/// Represents a text chat room in a discord server or private message
struct Channel {}

impl Channel {
    pub async fn send(&self, message: &str) {
        // send a message for this channel to the discord api
    }
}

/// A Guild is discord's name for servers
struct Guild {}

/// The user that sent the message or command
struct User {}
