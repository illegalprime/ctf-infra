extern crate serde_json;
extern crate curl;

use super::super::Object;
use super::Interactive;
use self::serde_json::Value;
use self::curl::easy::Easy;
use std::io::{
    self,
    Write,
};
use std::process::Command;
use std::fs::File;

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

            if let Ok(preview) = preview_image(url) {
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

fn preview_image(url: &str) -> Result<String, &'static str> {
    // Make a file to hold the image
    let image_path = format!("/tmp/{}", url);

    // Get the image
    {
        let mut image = File::create(&image_path).unwrap();
        let mut fetch = Easy::new();
        if fetch.url(url).is_err() {
            return Err("Not a valid URL");
        }
        fetch.write_function(move |data| {
            Ok(image.write(data).unwrap())
        }).unwrap();
        fetch.perform().unwrap();
    }

    let output = Command::new("img2txt")
        .arg(&image_path)
        .output()
        .unwrap();

    let text = String::from_utf8_lossy(&output.stdout[..]);

    Ok(text.into_owned())
}
