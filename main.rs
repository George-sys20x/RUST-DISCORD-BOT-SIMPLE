extern crate discord;

use discord::{Discord, State};
use discord::model::{Event, ChannelId};

fn main() {
    let discord = Discord::new("my_token").expect("Error creating Discord instance");
    let (mut connection, ready) = discord.connect().expect("Error connecting to Discord");
    println!("[Ready] {} is serving {} servers", ready.user.username, ready.guilds.len());
    let mut state = State::new(ready);

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                let channel = message.channel_id;
                if message.content == "!ping" {
                    let _ = discord.send_message(channel, "Pong!", "", false);
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("[Error] Connection closed with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("[Error] Receive error: {:?}", err)
        }
    }
}
