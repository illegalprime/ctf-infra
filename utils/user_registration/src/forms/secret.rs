extern crate serde_json;
extern crate rpassword;

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
            let first_attempt = rpassword::read_password().unwrap();
            print!("{}", verify);
            io::stdout().flush().unwrap();
            let second_attempt = rpassword::read_password().unwrap();

            if first_attempt == second_attempt {
                return Value::String(first_attempt);
            } else {
                println!("{}", error);
            }
        }
    }
}
