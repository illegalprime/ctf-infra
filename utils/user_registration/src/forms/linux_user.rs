extern crate serde_json;

use super::super::Object;
use super::Interactive;

pub struct Handler;

impl Interactive for Handler {
    fn interact(opts: &Object) {
        unimplemented!();
    }
}
