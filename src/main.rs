extern crate serenity;
extern crate url;

use serenity::prelude::*;
use serenity::model::channel::Message;

use url::Url;

use std::env;

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(
            &env::var("DISCORD_TOKEN").expect("missing discord token"), Handler
        ).expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if contains_invite_link(&msg.content) {
            let _ = msg.channel_id.say("Aye, don't do that here kids!");
        }
    }
}

const DISCORD_HOSTNAMES: [&str; 2] = ["discord.me", "discord.gg"];
// TODO: Should this be an env var or something?
const DISCORD_RUST_SERVER_INVITE: &str = "/rust-lang";

/// Check if a link is a discord 
fn contains_invite_link(content: &str) -> bool {
    if let Ok(url) = Url::parse(content) {
        // this may not be fool proof but should be good enough for now!
        // ignores invites to the rust server server
        if let Some(host_str) = url.host_str() {
            DISCORD_HOSTNAMES.contains(&host_str) && url.path() != DISCORD_RUST_SERVER_INVITE
        } else {
            // if the host_str is None then it's just a link to the discord website
            false
        }
    } else {
        false
    }
}
