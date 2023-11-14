use crate::common::bool_from_int;
use crate::user::ContentAuthor;
use getset::{CopyGetters, Getters};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Debug, Getters, CopyGetters)]
pub struct Machine {
    #[get_copy = "pub"]
    id: u32,
    #[get = "pub"]
    name: String,
    // TODO: make this an enum
    #[get = "pub"]
    os: String,
    #[serde(deserialize_with = "bool_from_int")]
    #[get_copy = "pub"]
    active: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[get_copy = "pub"]
    retired: bool,
    #[get_copy = "pub"]
    points: u32,
    #[get_copy = "pub"]
    static_points: u32,
    // TODO: make this a DateTime
    #[get = "pub"]
    release: String,
    #[serde(rename = "user_owns_count")]
    #[get_copy = "pub"]
    user_owns: u32,
    #[serde(rename = "root_owns_count")]
    #[get_copy = "pub"]
    root_owns: u32,
    #[get_copy = "pub"]
    free: bool,
    #[get_copy = "pub"]
    stars: f32,
    #[get_copy = "pub"]
    difficulty: u32,
    #[get = "pub"]
    avatar: String,
    #[serde(rename = "feedbackForChart")]
    // TODO: feedback chart object
    #[get = "pub"]
    feedback: serde_json::Value,
    #[serde(rename = "difficultyText")]
    // TODO: make an enum
    #[get = "pub"]
    difficulty_text: String,
    #[serde(rename = "isCompleted")]
    #[get_copy = "pub"]
    is_completed: bool,
    // TODO: maker object
    #[get = "pub"]
    #[serde(deserialize_with = "crate::user::maker")]
    maker: ContentAuthor,
    #[get = "pub"]
    #[serde(deserialize_with = "crate::user::maker_optional")]
    maker2: Option<ContentAuthor>,
    #[serde(deserialize_with = "bool_from_int")]
    #[get_copy = "pub"]
    recommended: bool,
    #[get_copy = "pub"]
    is_competitive: bool,
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Machine `{}`", self.name)
    }
}
