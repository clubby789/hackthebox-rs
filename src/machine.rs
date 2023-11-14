use crate::common::bool_from_int;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct Machine {
    id: u32,
    name: String,
    // TODO: make this an enum
    os: String,
    #[serde(deserialize_with = "bool_from_int")]
    active: bool,
    #[serde(deserialize_with = "bool_from_int")]
    retired: bool,
    points: u32,
    static_points: u32,
    // TODO: make this a DateTime
    release: String,
    #[serde(rename = "user_owns_count")]
    user_owns: u32,
    #[serde(rename = "root_owns_count")]
    root_owns: u32,
    free: bool,
    stars: f32,
    difficulty: u32,
    avatar: String,
    #[serde(rename = "feedbackForChart")]
    // TODO: feedback chart object
    feedback: serde_json::Value,
    #[serde(rename = "difficultyText")]
    // TODO: make an enum
    difficulty_text: String,
    #[serde(rename = "isCompleted")]
    is_completed: bool,
    // TODO: maker object
    maker: Option<serde_json::Value>,
    maker2: Option<serde_json::Value>,
    #[serde(deserialize_with = "bool_from_int")]
    recommended: bool,
    is_competitive: bool,
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Machine `{}`", self.name)
    }
}
