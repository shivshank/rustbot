extern crate serenity;
extern crate url;

use serenity::prelude::*;
use serenity::model::channel::Message;

use url::Url;

use std::env;
use std::path::Path;

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
        // TODO: Make a list of filters and iterate through it
        if contains_invite_link(&msg.content) {
            let _ = msg.channel_id.say("No advertising.");
        } else if contains_image_link(&msg) {
            let _ = msg.channel_id.say("You posted an image!");
        }
    }
}

const DISCORD_ADVERTISING_HOSTNAMES: [&str; 2] = ["discord.me", "discord.gg"];
// TODO: Should this be an env var or something?
const DISCORD_RUST_SERVER_INVITE: &str = "/rust-lang";

/// Check if a link is a discord 
fn contains_invite_link(content: &str) -> bool {
    // TODO: Check for links within a message's content
    if let Ok(url) = Url::parse(content) {
        // ignores invites to the rust server server
        if let Some(host_str) = url.host_str() {
            DISCORD_ADVERTISING_HOSTNAMES.contains(&host_str) 
                && url.path() != DISCORD_RUST_SERVER_INVITE
        } else {
            // if the host_str is None then it's just a link to the discord website
            false
        }
    } else {
        false
    }
}

const IMAGE_SITES: [&str; 2] = ["imgur.com", "cdn.discordapp.com"];
const IMAGE_EXTENSIONS: [&str; 4] = [".gif", ".jpg", ".jpeg", ".png"];

fn contains_image_link(msg: &Message) -> bool {
    // TODO: When we decide on an image API this should probably just do the request
    // since it does extra work to decide which URL makes it an "image"

    if msg.embeds.len() == 0 {
        // check the message content
        // TODO: Search for a link within the content
        return if let Ok(url) = Url::parse(&msg.content) {
            link_is_recognized_image_format(&url)
        } else {
            false
        };
    }

    for embed in &msg.embeds {
        // in this case since discord has apparently done some extra work for us we will assume the
        // link is actually an image instead of trying to analyze the link
        if embed.kind == "image" {
            return true;
        } else if let Some(_) = embed.thumbnail {
            return true;
        }
    }

    return false;
}

fn link_is_recognized_image_format(url: &Url) -> bool {
    url.host_str()
        .and_then(|host_str| if IMAGE_SITES.contains(&host_str) { Some(()) } else { None })
        .or_else(|| {
            // TODO: This seems really gross. Clean up if possible.
            let ext = Path::new(url.path()).extension()?;
            for known_img_ext in &IMAGE_EXTENSIONS {
                // unwrap should never fail since ext comes from a String!
                if ext.to_str().unwrap() == *known_img_ext {
                    return Some(())
                }
            }
            return None
        })
        .is_some()
}
