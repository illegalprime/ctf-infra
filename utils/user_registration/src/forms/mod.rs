pub mod secret;
pub mod linux_user;
pub mod image;

use super::Object;

pub trait Interactive {
    fn interact(opts: &Object);
}
