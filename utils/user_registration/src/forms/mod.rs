extern crate serde_json;

pub mod secret;
pub mod linux_user;
pub mod image;

use super::Object;
use serde_json::Value;

pub trait Interactive {
    fn interact(opts: &Object) -> Value;
}
