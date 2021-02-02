#[macro_use]
extern crate log;

mod client;
mod mailer;
mod transactional;

pub use mailer::Mailer;
pub use transactional::*;

pub type Sendinblue = client::Client;
