extern crate serde_json;

use super::super::Object;
use super::Interactive;
use serde_json::Value;

pub struct Handler;

impl Interactive for Handler {
    fn interact(opts: &Object) -> Value {
        unimplemented!();
    }
}
