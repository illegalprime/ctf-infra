extern crate serde_json;

mod forms;

use serde_json::Value;
use std::collections::BTreeMap;
use forms::Interactive;

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

    for question in questions.iter() {
        (match question.get("type").and_then(Value::as_string) {
            Some("image") => forms::image::Handler::interact,
            Some("linux_user") => forms::linux_user::Handler::interact,
            Some("secret") => forms::secret::Handler::interact,
            Some(t) => panic!("Question type '{}' is not a valid type", t),
            None => panic!("each question must contain a `type` field"),
        })(question);
    }
}
