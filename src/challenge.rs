use crate::common::{bool_from_int, bool_from_option_bool};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct Challenge {
    id: u32,
    name: String,
    #[serde(deserialize_with = "bool_from_int")]
    retired: bool,
    // TODO: make an enum
    #[serde(rename = "difficulty")]
    difficulty: String,
    #[serde(rename = "difficulty_chart")]
    // TODO: difficulty chart object
    difficulty_chart: serde_json::Value,
    solves: u32,
    #[serde(rename = "authUserSolveTime")]
    auth_user_solve_time: String,
    likes: u32,
    dislikes: u32,
    stars: f32,
    description: String,
    // TODO: make an enum
    category_name: String,
    // TODO: Blood object?
    first_blood_user_id: u32,
    first_blood_user: String,
    first_blood_time: String,
    first_blood_user_avatar: String,
    // TODO: MakerSummary?
    creator_id: u32,
    creator_name: String,
    creator_avatar: String,
    #[serde(rename = "isRespected")]
    is_respected: bool,

    creator2_id: Option<u32>,
    creator2_name: Option<String>,
    creator2_avatar: Option<String>,
    #[serde(rename = "isRespected2")]
    is_respected_2: Option<bool>,

    #[serde(deserialize_with = "bool_from_option_bool", rename = "download")]
    has_download: bool,
    sha256: String,

    #[serde(deserialize_with = "bool_from_option_bool", rename = "docker")]
    has_docker: bool,
    // TODO: DockerInstance
    docker_ip: Option<String>,
    docker_port: Option<u16>,

    // TODO: DateTime
    release_date: String,
    #[serde(deserialize_with = "bool_from_int")]
    released: bool,

    #[serde(rename = "authUserSolve")]
    auth_user_solve: bool,
    #[serde(rename = "likeByAuthUser")]
    like_by_auth_user: bool,
    #[serde(rename = "dislikeByAuthUser")]
    dislike_by_auth_user: bool,
    #[serde(rename = "isTodo")]
    is_todo: bool,
    #[serde(deserialize_with = "bool_from_int")]
    recommended: bool,
    #[serde(rename = "authUserHasReviewed")]
    auth_user_has_reviewed: bool,
    user_can_review: bool,
}

impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Challenge `{}`", self.name)
    }
}
