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

#[cfg(test)]
mod test {
  use crate::*;

  use dotenv::dotenv;
  use serde::Serialize;

  #[derive(Debug, Serialize)]
  struct Required {
    brand_name: String,
    banner: String,
    homepage_link: String,
    facebook_link: String,
    instagram_link: String,
  }

  #[derive(Debug, Serialize)]
  struct OrderList {
    list_name: String,
    orders: Vec<Order>,
  }

  #[derive(Debug, Serialize, Clone)]
  struct Order {
    order_number: String,
    address: String,
  }

  #[derive(Debug, Serialize, Clone)]
  struct OneOrder {
    order: Order,
    count: i32,
  }

  #[test]
  fn test_transactional_body_with_add_values() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let sender = Mailer {
      name: "sender_name".to_string(),
      email: "sender_email".to_string(),
    };

    let receiver = Mailer {
      name: "receiver_name".to_string(),
      email: "receiver_email".to_string(),
    };

    let required = Required {
      brand_name: "brand_name".to_string(),
      banner: "banner".to_string(),
      homepage_link: "homepage_link".to_string(),
      facebook_link: "facebook_link".to_string(),
      instagram_link: "instagram_link".to_string(),
    };

    let old_payload = TransactionalBody::builder()
      .set_sender(sender.clone())
      .add_to_mailer(receiver.clone())
      .reply_to(sender.clone())
      .template_id(36)
      .subject("TEST SENDINBLUE".to_string())
      .add_params("brand_name", required.brand_name.clone())
      .add_params("banner", required.banner.clone())
      .add_params("homepage_link", required.homepage_link.clone())
      .add_params("facebook_link", required.facebook_link.clone())
      .add_params("instagram_link", required.instagram_link.clone())
      .create();

    debug!("old_payload: {:?}", old_payload);

    let new_payload = TransactionalBody::builder()
      .set_sender(sender.clone())
      .add_to_mailer(receiver)
      .reply_to(sender)
      .template_id(36)
      .subject("TEST SENDINBLUE".to_string())
      .add_values(serde_json::to_value(required).unwrap())
      .create();

    debug!("new_payload: {:?}", new_payload);

    assert_eq!(old_payload.params, new_payload.params);
  }

  #[test]
  fn test_transactional_body_with_add_values_and_array() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let sender = Mailer {
      name: "sender_name".to_string(),
      email: "sender_email".to_string(),
    };

    let receiver = Mailer {
      name: "receiver_name".to_string(),
      email: "receiver_email".to_string(),
    };

    let required = Required {
      brand_name: "brand_name".to_string(),
      banner: "banner".to_string(),
      homepage_link: "homepage_link".to_string(),
      facebook_link: "facebook_link".to_string(),
      instagram_link: "instagram_link".to_string(),
    };

    let order_1 = Order {
      order_number: "YGORDER1".to_string(),
      address: "ADDRESS1".to_string(),
    };

    let order_2 = Order {
      order_number: "YGORDER2".to_string(),
      address: "ADDRESS2".to_string(),
    };

    let order_list = OrderList {
      list_name: "TESTORDERLIST".to_string(),
      orders: vec![order_1, order_2],
    };

    let payload = TransactionalBody::builder()
      .set_sender(sender.clone())
      .add_to_mailer(receiver)
      .reply_to(sender)
      .template_id(36)
      .subject("TEST SENDINBLUE".to_string())
      .add_values(serde_json::to_value(required).unwrap())
      .add_values(serde_json::to_value(order_list).unwrap())
      .create();

    debug!("payload: {:?}", payload);
  }

  #[test]
  fn test_transactional_body_with_add_values_and_object() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let sender = Mailer {
      name: "sender_name".to_string(),
      email: "sender_email".to_string(),
    };

    let receiver = Mailer {
      name: "receiver_name".to_string(),
      email: "receiver_email".to_string(),
    };

    let required = Required {
      brand_name: "brand_name".to_string(),
      banner: "banner".to_string(),
      homepage_link: "homepage_link".to_string(),
      facebook_link: "facebook_link".to_string(),
      instagram_link: "instagram_link".to_string(),
    };

    let order_1 = Order {
      order_number: "YGORDER1".to_string(),
      address: "ADDRESS1".to_string(),
    };

    let order = OneOrder {
      order: order_1,
      count: 2,
    };

    let payload = TransactionalBody::builder()
      .set_sender(sender.clone())
      .add_to_mailer(receiver)
      .reply_to(sender)
      .template_id(36)
      .subject("TEST SENDINBLUE".to_string())
      .add_values(serde_json::to_value(required).unwrap())
      .add_values(serde_json::to_value(order).unwrap())
      .create();

    debug!("payload: {:?}", payload);
  }
}
