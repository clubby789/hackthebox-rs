use crate::common::{bool_from_int, bool_from_option_bool};
use crate::user::ContentAuthor;
use getset::{CopyGetters, Getters};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Debug, CopyGetters, Getters)]
pub struct Challenge {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,

    #[serde(deserialize_with = "bool_from_int")]
    #[getset(get_copy = "pub")]
    retired: bool,
    // TODO: make an enum
    #[serde(rename = "difficulty")]
    #[getset(get = "pub")]
    difficulty: String,
    #[serde(rename = "difficulty_chart")]
    // TODO: difficulty chart object
    #[getset(get = "pub")]
    difficulty_chart: serde_json::Value,
    #[getset(get_copy = "pub")]
    solves: u32,
    #[serde(rename = "authUserSolveTime")]
    #[getset(get = "pub")]
    auth_user_solve_time: Option<String>,
    #[getset(get_copy = "pub")]
    likes: u32,
    #[getset(get_copy = "pub")]
    dislikes: u32,
    #[getset(get_copy = "pub")]
    stars: f32,
    #[getset(get = "pub")]
    description: String,
    // TODO: make an enum
    #[getset(get = "pub")]
    category_name: String,
    // TODO: Blood object?
    #[getset(get_copy = "pub")]
    first_blood_user_id: u32,
    #[getset(get = "pub")]
    first_blood_user: String,
    #[getset(get = "pub")]
    first_blood_time: String,
    #[getset(get = "pub")]
    first_blood_user_avatar: String,

    #[getset(get = "pub")]
    #[serde(flatten)]
    author: ContentAuthor,
    #[getset(get = "pub")]
    #[serde(flatten, deserialize_with = "crate::user::content_author_2")]
    author2: Option<ContentAuthor>,

    #[serde(deserialize_with = "bool_from_option_bool", rename = "download")]
    #[getset(get_copy = "pub")]
    has_download: bool,
    #[getset(get = "pub")]
    sha256: String,

    #[serde(deserialize_with = "bool_from_option_bool", rename = "docker")]
    #[getset(get_copy = "pub")]
    has_docker: bool,
    // TODO: DockerInstance
    #[getset(get = "pub")]
    docker_ip: Option<String>,
    #[getset(get_copy = "pub")]
    docker_port: Option<u16>,

    // TODO: DateTime
    #[getset(get = "pub")]
    release_date: String,
    #[serde(deserialize_with = "bool_from_int")]
    #[getset(get_copy = "pub")]
    released: bool,

    #[serde(rename = "authUserSolve")]
    #[getset(get_copy = "pub")]
    auth_user_solve: bool,
    #[serde(rename = "likeByAuthUser")]
    #[getset(get_copy = "pub")]
    like_by_auth_user: bool,
    #[serde(rename = "dislikeByAuthUser")]
    #[getset(get_copy = "pub")]
    dislike_by_auth_user: bool,
    #[serde(rename = "isTodo")]
    #[getset(get_copy = "pub")]
    is_todo: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[getset(get_copy = "pub")]
    recommended: bool,
    #[serde(rename = "authUserHasReviewed")]
    #[getset(get_copy = "pub")]
    auth_user_has_reviewed: bool,
    #[getset(get_copy = "pub")]
    user_can_review: bool,
}

impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Challenge `{}`", self.name)
    }
}
