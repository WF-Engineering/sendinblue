use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{mailer::Mailer, Sendinblue};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionalBody {
  pub(crate) sender: Mailer,
  pub(crate) to: Vec<Mailer>,
  pub(crate) reply_to: Mailer,
  pub(crate) template_id: u32,
  pub(crate) subject: String,
  pub(crate) params: Value,
}

impl TransactionalBody {
  pub fn builder() -> TransactionalBodyBuilder {
    let inner = TransactionalBody::default();
    TransactionalBodyBuilder(inner)
  }
}

pub struct TransactionalBodyBuilder(TransactionalBody);

impl TransactionalBodyBuilder {
  pub fn set_sender(self, mailer: Mailer) -> Self {
    let mut inner = self.0;
    inner.sender = mailer;
    Self(inner)
  }

  pub fn add_to_mailer(self, mailer: Mailer) -> Self {
    let mut inner = self.0;
    inner.to.push(mailer);
    Self(inner)
  }

  pub fn reply_to(self, mailer: Mailer) -> Self {
    let mut inner = self.0;
    inner.reply_to = mailer;
    Self(inner)
  }

  pub fn template_id(self, template_id: u32) -> Self {
    let mut inner = self.0;
    inner.template_id = template_id;
    Self(inner)
  }

  pub fn subject<S>(self, subject: S) -> Self
  where
    S: Into<String>,
  {
    let mut inner = self.0;
    inner.subject = subject.into();
    Self(inner)
  }

  pub fn add_params<S>(self, key: &str, value: S) -> Self
  where
    S: Into<String>,
  {
    let mut inner = self.0;
    inner.params[key] = Value::String(value.into());
    Self(inner)
  }

  pub fn add_params_array<A>(self, key: &str, value: Vec<A>) -> Self
  where
    A: Serialize,
  {
    let mut inner = self.0;
    inner.params[key] = serde_json::to_value(value).unwrap();
    Self(inner)
  }

  pub fn create(self) -> TransactionalBody {
    self.0
  }

  pub fn add_values(self, values: Value) -> Self {
    let mut inner = self.0;

    if let Value::Object(values) = values {
      for (key, value) in values {
        match value {
          Value::Null => {
            continue;
          }
          Value::Bool(b) => {
            inner.params[key] = Value::Bool(b);
          }
          Value::Number(n) => {
            inner.params[key] = Value::Number(n);
          }
          Value::String(s) => {
            inner.params[key] = Value::String(s);
          }
          Value::Array(a) => {
            inner.params[key] = serde_json::to_value(a).unwrap();
          }
          Value::Object(o) => {
            inner.params[key] = serde_json::to_value(o).unwrap();
          }
        }
      }
    };
    Self(inner)
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionalResp {
  pub message_id: String,
}

impl Sendinblue {
  pub async fn send_transactional_email(
    &self,
    body: TransactionalBody,
  ) -> Result<TransactionalResp, reqwest::Error> {
    debug!(
      "send_transactional_email: {}",
      serde_json::to_string_pretty(&body).unwrap()
    );

    let url = format!("{}/smtp/email", self.server_url);

    reqwest::Client::new()
      .post(&url)
      .json(&body)
      .send()
      .await?
      .json()
      .await
  }
}
