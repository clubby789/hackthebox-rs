use crate::common::{bool_from_int, bool_from_option_bool};
use crate::user::ContentAuthor;
use getset::{CopyGetters, Getters};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Debug, CopyGetters, Getters)]
pub struct Challenge {
    #[get_copy = "pub"]
    id: u32,
    #[get = "pub"]
    name: String,

    #[serde(deserialize_with = "bool_from_int")]
    #[get_copy = "pub"]
    retired: bool,
    // TODO: make an enum
    #[serde(rename = "difficulty")]
    #[get = "pub"]
    difficulty: String,
    #[serde(rename = "difficulty_chart")]
    // TODO: difficulty chart object
    #[get = "pub"]
    difficulty_chart: serde_json::Value,
    #[get_copy = "pub"]
    solves: u32,
    #[serde(rename = "authUserSolveTime")]
    #[get = "pub"]
    auth_user_solve_time: Option<String>,
    #[get_copy = "pub"]
    likes: u32,
    #[get_copy = "pub"]
    dislikes: u32,
    #[get_copy = "pub"]
    stars: f32,
    #[get = "pub"]
    description: String,
    // TODO: make an enum
    #[get = "pub"]
    category_name: String,
    // TODO: Blood object?
    #[get_copy = "pub"]
    first_blood_user_id: u32,
    #[get = "pub"]
    first_blood_user: String,
    #[get = "pub"]
    first_blood_time: String,
    #[get = "pub"]
    first_blood_user_avatar: String,

    #[get = "pub"]
    #[serde(flatten)]
    author: ContentAuthor,
    #[get = "pub"]
    #[serde(flatten, deserialize_with = "crate::user::content_author_2")]
    author2: Option<ContentAuthor>,

    #[serde(deserialize_with = "bool_from_option_bool", rename = "download")]
    #[get_copy = "pub"]
    has_download: bool,
    #[get = "pub"]
    sha256: String,

    #[serde(deserialize_with = "bool_from_option_bool", rename = "docker")]
    #[get_copy = "pub"]
    has_docker: bool,
    // TODO: DockerInstance
    #[get = "pub"]
    docker_ip: Option<String>,
    #[get_copy = "pub"]
    docker_port: Option<u16>,

    // TODO: DateTime
    #[get = "pub"]
    release_date: String,
    #[serde(deserialize_with = "bool_from_int")]
    #[get_copy = "pub"]
    released: bool,

    #[serde(rename = "authUserSolve")]
    #[get_copy = "pub"]
    auth_user_solve: bool,
    #[serde(rename = "likeByAuthUser")]
    #[get_copy = "pub"]
    like_by_auth_user: bool,
    #[serde(rename = "dislikeByAuthUser")]
    #[get_copy = "pub"]
    dislike_by_auth_user: bool,
    #[serde(rename = "isTodo")]
    #[get_copy = "pub"]
    is_todo: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[get_copy = "pub"]
    recommended: bool,
    #[serde(rename = "authUserHasReviewed")]
    #[get_copy = "pub"]
    auth_user_has_reviewed: bool,
    #[get_copy = "pub"]
    user_can_review: bool,
}

impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Challenge `{}`", self.name)
    }
}
