use std::fmt::Display;

use serde::Deserialize;

use crate::{
    Challenge,
    HackTheBoxError::{Http, JSONParseError, RequiresAuthentication},
    Machine, Result, User,
};

pub const API_BASE: &str = "https://www.hackthebox.com/api/v4/";
pub const USER_AGENT: &str = concat!("hackthebox-rs/", env!("CARGO_PKG_VERSION"));

/// The entrypoint client for accessing the Hack The Box API.
#[derive(Default)]
pub struct HackTheBox {
    token: Option<String>,
    http: reqwest::Client,
}

impl HackTheBox {
    /// Create a new, unauthenticated [`HackTheBox`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new, authenticated [`HackTheBox`] using an app token.
    /// You can generate an app token in your [profile settings](https://app.hackthebox.com/profile/settings)
    pub fn new_authenticated(token: String) -> Self {
        Self {
            token: Some(token),
            ..Default::default()
        }
    }

    /// Add an app token to an existing client
    pub fn authenticate(&mut self, token: String) {
        self.token = Some(token);
    }

    /// Send a GET request to the given endpoint (the API base URL should not be included).
    /// If `authenticated` is set, an authentication token is sent
    /// The resulting JSON data will be parsed as a given type.
    pub async fn do_get_request<Res>(&self, endpoint: &str, authenticated: bool) -> Result<Res>
    where
        for<'de> Res: Deserialize<'de>,
    {
        let mut req = self
            .http
            .get(format!("{API_BASE}{endpoint}"))
            .header(reqwest::header::USER_AGENT, USER_AGENT);
        if authenticated {
            let Some(token) = &self.token else {
                return Err(RequiresAuthentication);
            };
            req = req.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
        }
        let resp = match req.send().await {
            Ok(resp) => resp,
            Err(e) => {
                return Err(Http(e));
            }
        };
        resp.json::<Res>().await.map_err(|e| JSONParseError(e))
    }

    /// Get a given [`Machine`] from the API. `id` should be either the machine name
    /// or numeric ID.
    pub async fn get_machine(&self, id: impl Display) -> Result<Machine> {
        self.do_get_request::<InfoWrapper<_>>(&format!("machine/info/{id}"), true)
            .await
            .map(|w| w.info)
    }

    /// Get a given [`User`] from the API.
    pub async fn get_user(&self, id: u32) -> Result<User> {
        self.do_get_request::<ProfileWrapper<_>>(&format!("user/profile/basic/{id}"), true)
            .await
            .map(|w| w.profile)
    }

    /// Get a given [`Challenge`] from the API.
    pub async fn get_challenge(&self, id: u32) -> Result<Challenge> {
        self.do_get_request::<ProfileWrapper<_>>(&format!("challenge/info/{id}"), true)
            .await
            .map(|w| w.profile)
    }
}

#[derive(Deserialize)]
struct InfoWrapper<T> {
    info: T,
}

#[derive(Deserialize)]
struct ProfileWrapper<T> {
    profile: T,
}
