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
use std::error::Error;
use std::fs::{
    self,
    File,
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

            println!("Please wait. Fetching image preview....");

            if let Ok(preview) = preview_image(url) {
                print!("{}", preview);
                io::stdout().flush().unwrap();
            } else {
                println!("{} ('{}')", error, url);
            }

            // Keep trying until we get an explicit yes or no
            loop  {
                print!("{}", verify);
                io::stdout().flush().unwrap();

                let mut response = String::new();
                io::stdin().read_line(&mut response).unwrap();
                match response.trim().chars().next().and_then(|c| c.to_lowercase().next()) {
                    Some(c) if c == 'y' => return Value::String(url.to_string()),
                    Some(c) if c == 'n' => break,
                    _ => continue,
                };
            }
        }
    }
}

fn preview_image(url: &str) -> Result<String, &'static str> {
    // Make a file to hold the image
    let image_path = format!("/tmp/user_register_{}", url.replace("/", "_"));

    let cleanup = || {
        fs::remove_file(&image_path).ok();
    };

    // Get the image
    {
        let mut image = File::create(&image_path)
            .expect("Could not create temporary profile pic file");

        let mut fetch = Easy::new();

        if fetch.url(url).is_err() {
            cleanup();
            return Err("Not a valid URL");
        }

        fetch.write_function(move |data| {
            Ok(image.write(data).expect("Could not write to pic file"))
        })
        .expect("Could not set write function of curl.");

        if let Err(e) = fetch.perform() {
            cleanup();
            println!("{}", e.description());
            return Err("Could not fetch the image.");
        }
    }

    let output = Command::new("img2txt")
        .arg(&image_path)
        .output()
        .expect("Could not get output from img2txt");

    cleanup();

    let text = String::from_utf8(output.stdout)
        .expect("img2txt emitted non utf8 data!");

    Ok(text)
}
