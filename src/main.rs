#![feature(convert)]
#![feature(collections)]
extern crate hyper;
extern crate rustc_serialize;

use std::io;
use std::io::prelude::*;
use std::process::Command;
use rustc_serialize::json::{Json, self};

use hyper::Client;

#[derive(RustcDecodable, Debug)]
struct Channel {
    name: String,
    meta_game: String,
    title: String,
    api: String,
    current_viewers: u32,
}

fn main() {
    let streamlist = populate_list(get_api());
    let sel_stream = prompt_user(streamlist);
    spawn_livestreamer(&sel_stream);
}

fn prompt_user (streams: Vec<Channel>) -> String {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut sel_stream = String::new();
    loop {
        write!(&mut stdout, "Select stream number > ");
        stdout.flush();

        let mut input = String::new();
        stdin.read_line(&mut input);
        let input_opt: Option<usize> = input.trim_right().parse::<usize>().ok();
        let input_int = match input_opt {
            Some(input_int) => input_int,
            None => panic!("{} is the wrong number!", input.trim_right()),
        };
        // input_int selects stream ID and returns stream url
        if input_int-1 <= streams.len() {
            let stream = &streams[input_int-1];
            sel_stream = format!("{}.tv/{}", stream.api, stream.name);
            break;
        }
    }
    sel_stream
}

fn populate_list(api_response: Json) -> Vec<Channel> {
    let channels = api_response
                    .as_object()
                    .and_then(|object|
                        object.get("_source")
                        )
                    .and_then(|source|
                        source.as_object()
                    )
                    .and_then(|source|
                        source.get("channels")
                    )
                    .and_then(|channels|
                        channels.as_array()
                    );
    let mut streams = Vec::new();
    if let Some(channels) = channels {
        for channel in channels {
            let why = json::encode(channel).unwrap();
            let channel:Channel = json::decode(&why[..]).unwrap();
            streams.push(channel);
        }
        streams.sort_by(|b, a| a.current_viewers.cmp(&b.current_viewers));
        for (i, stream) in streams.iter().enumerate() {
            println!("[{id}] {name} is playing {game}\n\t{title} | Viewers: {viewers}",
                        id=i+1, name=stream.name, game=stream.meta_game,
                        title=stream.title, viewers=stream.current_viewers);
        }
    }
    streams
}

fn get_api() -> Json {
    let mut client = Client::new();
    let mut res = client.get("http://api.speedrunslive.com/frontend/streams")
                  .send().unwrap();
    let mut v = Vec::new();
    res.read_to_end(&mut v).unwrap();
    let dicks = String::from_utf8_lossy(&v[..]);
    let data = Json::from_str(&dicks).unwrap();
    data
}

fn spawn_livestreamer(stream: &str) {
    println!("Let's watch {}!", stream);
    Command::new("livestreamer")
                      .arg(stream)
                      .arg("medium,source")
                      .output()
                      .unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
    });
}
