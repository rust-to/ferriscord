// Basic Discord bot using ferriscord
use ferriscord::{Connection, Command, Event};
use dotenv::dotenv;

fn main() {
    //check for a .env file
    dotenv().ok();
    //get the bot api token from the environment
    let bot_token = std::env::var("FERRIS_BOT_TOKEN").expect("FERRIS_BOT_TOKEN must be set");
    // Initialize a new bot connection
    let con = Connection::new(bot_token)
        .with_prefix('!') // the prefix to listen to for commands
        .with_default_help() // a default auto generated !help command
        .with_commands(say, ping) // register the bot's commands
        .with_events(on_ready, on_message) // register all event callbacks
        .run(); // start the bot

    // The broadcast_message command sends a message
    // to the default channel of every connected server
    con.broadcast_message("hello world!");
}

#[derive(Command)]
async fn say(ctx: Context) {
    let mut buf = "";
    let to_say: &str = ctx.message;
    ferris_says::say(to_say, to_say.len(), &mut buf).unwrap();
    await!(ctx.send(buf));
}

#[derive(Command)]
async fn ping(ctx: discord_api::Context) {
    await!(ctx.send("pong"));
}

#[derive(Event)]
async fn on_ready(ctx: discord_api::Context) {
    println!("Bot user {} is ready!",);
}

#[derive(Event)]
async fn on_message(ctx: discord_api::Context) {
    await!(ctx.send("hi!"));
}
