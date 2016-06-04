extern crate serde_json;

mod forms;

use serde_json::Value;
use std::collections::BTreeMap;
use forms::Interactive;
use std::net::TcpStream;
use std::io::{
    Write,
    Read,
};

pub type Object = BTreeMap<String, Value>;


fn main() {
    let config: Value = serde_json::from_str(include_str!("../config.json"))
        .expect("Config file is not valid JSON.");

    let config = config.as_object()
        .expect("Config file must be wrapped in a JSON Object.");

    let questions: Vec<&Object> = config.get("questions")
        .and_then(Value::as_array)
        .expect("'questions' field must be an array of objects")
        .iter()
        .map(Value::as_object)
        .map(|o| o.expect("each question must be a JSON Object."))
        .collect();

    let greeting = config.get("greeting")
        .and_then(Value::as_string);

    if let Some(greet_text) = greeting {
        println!("{}", greet_text);
    }

    let mut response = BTreeMap::new();

    for question in questions.iter() {
        let val = match question.get("type").and_then(Value::as_string) {
            Some("image") => forms::image::Handler::interact(question),
            Some("linux_user") => forms::linux_user::Handler::interact(question),
            Some("secret") => forms::secret::Handler::interact(question),
            Some("meta") => {
                let meta = question.get("value").and_then(Value::as_string)
                    .expect("meta must also have 'value' field");
                Value::String(meta.to_string())
            },
            Some(t) => panic!("Question type '{}' is not a valid type", t),
            None => panic!("each question must contain a `type` field"),
        };

        let id = question.get("id").and_then(Value::as_string)
            .expect("each question must have an id associated with it");

        response.insert(id, val);
    }

    let response = serde_json::to_string(&response).unwrap();

    let port = config.get("port").and_then(Value::as_u64)
        .expect("you must specify a port number to send your response to");

    if port > 65535 {
        panic!("your port number must be lower than 65535");
    }

    println!("Please wait. Sending data to server...");

    let mut connection = TcpStream::connect(("127.0.0.1", port as u16))
        .expect("Could not connect to the specified port");

    connection.write_all(response.as_bytes())
        .expect("Could not write data to socket");
    connection.write("\n".as_bytes())
        .expect("Could not write data to socket");
    connection.flush().ok();

    let mut reply = Vec::new();
    connection.read_to_end(&mut reply).ok();
    let reply = String::from_utf8(reply)
        .expect("Got non UTF-8 reply from the server!");
    let reply: Value = serde_json::from_str(&reply)
        .expect("got invalid JSON from server");
    let reply = reply.as_object()
        .expect("server reply must be object");

    match reply.get("status").and_then(Value::as_string) {
        Some("error") => {
            let description = reply.get("error").and_then(Value::as_string)
                .expect("expected an error description from the server");
            println!("Error from server: {}", description);
        },
        Some("ok") => {
            if let Some(farewell) = config.get("farewell").and_then(Value::as_string) {
                println!("{}", farewell);
            }
        },
        _ => panic!("invalid status from server"),
    };
}
