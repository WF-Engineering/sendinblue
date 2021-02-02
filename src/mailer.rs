use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Mailer {
  pub name: String,
  pub email: String,
}

impl Mailer {
  pub fn new<S>(name: S, email: S) -> Self
  where
    S: Into<String>,
  {
    Self {
      name: name.into(),
      email: email.into(),
    }
  }
}
