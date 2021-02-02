const BASE_URL: &str = "https://api.sendinblue.com/v3";

#[derive(Debug, Clone)]
pub struct Client {
  pub server_url: String,
  pub api_key: String,
}

impl Client {
  pub fn new(server_url: String, api_key: String) -> Self {
    Self {
      server_url,
      api_key,
    }
  }

  pub fn production(api_key: String) -> Self {
    Self {
      server_url: BASE_URL.to_string(),
      api_key,
    }
  }
}
