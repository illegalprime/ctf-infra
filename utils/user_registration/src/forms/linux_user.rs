extern crate serde_json;
extern crate users;
extern crate regex;

use super::super::Object;
use super::Interactive;
use self::serde_json::Value;
use self::regex::Regex;
use std::io::{
    self,
    Write,
};

pub struct Handler;

impl Interactive for Handler {
    fn interact(opts: &Object) -> Value {
        let prompt = opts.get("prompt").and_then(Value::as_string)
            .expect("The linux_user type requires a prompt");
        let invalid = opts.get("invalid").and_then(Value::as_string)
            .expect("The linux_user type requires a message for invalid input");
        let taken = opts.get("taken").and_then(Value::as_string)
            .expect("The linux_user type requires a message when the username is taken");

        let user_re = Regex::new("^[a-z_][a-z0-9_]{0,30}$").unwrap();
        let mut username = String::new();

        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();
            username.clear();
            io::stdin().read_line(&mut username).unwrap();
            let username = username.trim();

            if user_re.is_match(username) {
                if users::get_user_by_name(username).is_none() {
                    return Value::String(username.to_string());
                } else {
                    println!("{}", taken);
                }
            } else {
                println!("{}", invalid);
            }
        }
    }
}
