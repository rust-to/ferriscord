// Basic Discord bot using ferriscord

#![feature(async_await)]

mod ferris;

use ferriscord_derive;

use ferriscord::{Connection, Context};

//use dotenv::dotenv;

#[derive(DiscordBot)]//this derive will add all of the #[command] #[event] #[help] tagged functions automatically
struct MyBot;

impl MyBot {

    #[command]
    fn ping(ctx: Context<Command>) {

    }

    #[help]
    fn help(ctx: Context<Help>) {

    }

    #[event]
    fn on_message(ctx: Context<Event>) {

    }

}

fn main() {
    //check for a .env file
    //dotenv().ok();

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





//#[derive(Command)]
async fn say(ctx: Context) {
    let to_say: &str = ctx.message;
    ferris::say(to_say.as_bytes(), to_say.len()).unwrap();
    ctx.send(buf).await;
}

//#[derive(Command)]
async fn ping(ctx: Context) {
    ctx.send("pong").await;
}

//#[derive(Event)]
async fn on_ready(ctx: Context) {
    println!("Bot user {} is ready!");
}

//#[derive(Event)]
async fn on_message(ctx: Context) {
    async {ctx.send("hi!")}.await;
}
