#![feature(try_from)]

extern crate dotenv;
extern crate futures;
extern crate telegram_bot;
extern crate teletext;
extern crate tokio_core;

use dotenv::dotenv;
use futures::Stream;
use std::env;
use telegram_bot::prelude::*;
use telegram_bot::{Api, ParseMode, UpdateKind};
use telegram_bot::{MessageKind, MessageEntityKind};
use teletext::{Square, Star, Qstar, Sw};
use tokio_core::reactor::Core;

use std::convert::TryInto;

fn main() {
    dotenv().ok();
    let mut core = Core::new().unwrap();
    let token = env::var("TELETEXT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();
    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            let mut result = Err(());
            if let MessageKind::Text { ref data, ref entities } = message.kind {
                if let Some(first) = entities.first() {
                    // Check if the first token is MessageEntityKind::BotCommand,
                    // Offset is probably 0 because telegram trims the leading spaces
                    if first.kind != MessageEntityKind::BotCommand || first.offset != 0 {
                        return Ok(())
                    }

                    let (cmd, text) = data.split_at(first.length as usize);

                    result = match cmd {
                        "/square" => Ok(format!("{}", text.try_into().unwrap_or(Square{ raw: "goforkurself".into() }))),
                        "/star" => Ok(format!("{}", text.try_into().unwrap_or(Star{ raw: "goforkurself".into() }))),
                        "/qstar" => Ok(format!("{}", text.try_into().unwrap_or(Qstar{ raw: "goforkurself".into() }))),
                        "/sw" => Ok(format!("{}", text.try_into().unwrap_or(Sw{ raw: "goforkurself".into() }))),
                        _ => Err(())
                    };
                };
            };

            if let Ok(reply) = result {
                api.spawn(
                    message
                        .text_reply(format!("```\n{}\n```", reply))
                        .parse_mode(ParseMode::Markdown),
                );
            };
        }
        Ok(())
    });
    core.run(future).unwrap();
}
