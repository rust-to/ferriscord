/// Context data gets passed into callbacks for functions and events
/// provides useful objects such as the Message{User, Content}, Command{User, args}, the channel, server
pub struct Context<T>
    where T: CallbackContext
{
    data: T,
}

impl Context<Event> {
    fn send(&self, message: &str) {
        self.channel.send(message);
    }
}

/// The common things shared by a Context object
trait ContextData {
    fn send(&self, message: &str) {
        //self.channel.send(message);
    }
}

/// type passed into callback functions
pub trait CallbackContext {
    fn message(&self) -> Option<Message>;
    fn command(&self) -> Option<Command>;
    fn channel(&self) -> Option<Channel>;
    fn server(&self) -> Option<Guild>;
}

struct EventContext {}

impl CallbackContext for EventContext {
    fn message(&self) -> Option<Message> {
        None
    }
    fn command(&self) -> Option<Command> {
        None
    }
    fn channel(&self) -> Option<Channel> {
        None
    }
    fn server(&self) -> Option<Guild> {
        None
    }
}
