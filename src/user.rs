use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

// TODO: Make an enum
type RankType = String;

#[derive(Deserialize, Debug)]
pub struct User {
    id: u32,
    sso_id: u32,
    name: String,
    system_owns: u32,
    user_owns: u32,
    user_bloods: u32,
    system_bloods: u32,
    team: Option<TeamSummary>,
    respects: u32,
    rank: RankType,
    rank_id: u32,
    current_rank_progress: u32,
    next_rank: Option<RankType>,
    next_rank_points: Option<f32>,
    rank_ownership: u32,
    ranking: u32,
    avatar: String,
    timezone: String,
    #[serde(rename = "isVip")]
    is_vip: bool,
    #[serde(rename = "isDedicatedVip")]
    is_dedicated_vip: bool,
    public: bool,
    country_name: String,
    country_code: String,
    points: u32,
    university: Option<UniversitySummary>,
    university_name: Option<String>,
    description: Option<String>,
    github: Option<String>,
    linkedin: Option<String>,
    twitter: Option<String>,
    website: Option<String>,
    #[serde(rename = "isRespected")]
    is_respected: bool,
    #[serde(rename = "isFollowed")]
    is_followed: bool,
}

#[derive(Deserialize, Debug)]
pub struct TeamSummary {
    id: u32,
    name: String,
    ranking: u32,
    avatar: String,
}

#[derive(Deserialize, Debug)]
pub struct UniversitySummary {
    id: u32,
    name: String,
    logo_thumb_url: String,
    rank: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "User `{}`", self.name)
    }
}
