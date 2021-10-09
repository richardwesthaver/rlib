//! Auth Configs
use serde::{Serialize, Deserialize};
#[cfg(feature = "oauth")]
use yup_oauth2::ApplicationSecret;
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
pub struct AuthConfig {
  pub provider: AuthProvider,
  #[cfg(feature = "oauth")]
  pub oauth: Option<Oauth2Config>,
  #[cfg(feature = "ssh")]
  pub ssh: Option<SshConfig>,
  pub pw: Option<PasswordConfig>
}

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
pub struct AuthProvider(pub String,pub String);

impl fmt::Display for AuthProvider {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.0, self.1)
  }
}

impl FromStr for AuthProvider {
  type Err = ();
  fn from_str(input: &str) -> Result<AuthProvider, Self::Err> {
    let s: Vec<&str> = input.split(":").collect();
    Ok(AuthProvider(s[0].to_string(), s[1].to_string()))
  }
}

#[derive(Serialize, Deserialize, Default, Debug, Hash)]
pub struct PasswordConfig(String, String);

#[cfg(feature = "oauth")]
#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Clone, Default)]
pub struct Oauth2Config {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uris: Vec<String>,
  pub auth_uri: String,
  pub token_uri: String,
  pub project_id: Option<String>, //for apptoken
  pub client_email: Option<String>,
}

#[cfg(feature = "oauth")]
impl From<ApplicationSecret> for Oauth2Config {
  fn from(shh: ApplicationSecret) -> Self {
    Oauth2Config {
      client_id: shh.client_id,
      client_secret: shh.client_secret,
      redirect_uris: shh.redirect_uris,
      auth_uri: shh.auth_uri,
      token_uri: shh.token_uri,
      project_id: shh.project_id,
      client_email: shh.client_email,
    }
  }
}

#[cfg(feature = "oauth")]
impl From<Oauth2Config> for ApplicationSecret {
  fn from(cfg: Oauth2Config) -> Self {
    ApplicationSecret {
      client_id: cfg.client_id,
      client_secret: cfg.client_secret,
      redirect_uris: cfg.redirect_uris,
      auth_uri: cfg.auth_uri,
      token_uri: cfg.token_uri,
      project_id: cfg.project_id,
      client_email: cfg.client_email,
      ..Self::default()
    }
  }
}

#[cfg(feature = "ssh")]
#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Clone, Default)]
pub struct SshConfig {}

#[cfg(feature = "ssh")]
impl From<SshConfig> for thrussh::server::Config {
  fn from(cfg: SshConfig) -> Self {
    Self::default()
  }
}

#[cfg(feature = "ssh")]
impl From<SshConfig> for thrussh::client::Config {
  fn from(cfg: SshConfig) -> Self {
    Self::default()
  }
}
