extern crate serde_json;

use super::super::Object;
use super::Interactive;
use serde_json::Value;
use std::io::{
    self,
    Write,
};

pub struct Handler;

impl Interactive for Handler {
    fn interact(opts: &Object) -> Value {
        let prompt = opts.get("prompt").and_then(Value::as_string)
            .expect("The secret type requires a prompt");
        let verify = opts.get("verify").and_then(Value::as_string)
            .expect("The secret type requires a verification prompt");
        let error = opts.get("error").and_then(Value::as_string)
            .expect("The secret type requires an error message");

        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();
            let mut url = String::new();
            io::stdin().read_line(&mut url).unwrap();
            let url = url.trim();

            if let Some(preview) = preview_image(url) {
                println!("{}", preview);
            } else {
                println!("{} ('{}')", error, url);
            }
            println!("{}", verify);
            io::stdout().flush().unwrap();

            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();

            match response.chars().next().and_then(|c| c.to_lowercase().next()) {
                Some(c) if c == 'y' => return Value::String(url.to_string()),
                _ => continue,
            };
        }
    }
}

fn preview_image(url: &str) -> Option<String> {
    unimplemented!();
}
