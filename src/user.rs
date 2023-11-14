use getset::{CopyGetters, Getters};
use serde::{Deserialize, Deserializer};
use std::fmt::{self, Display, Formatter};

// TODO: Make an enum
type RankType = String;

#[derive(Deserialize, Debug, Getters, CopyGetters)]
pub struct User {
    #[get_copy = "pub"]
    id: u32,
    #[get_copy = "pub"]
    sso_id: u32,
    #[get = "pub"]
    name: String,
    #[get_copy = "pub"]
    system_owns: u32,
    #[get_copy = "pub"]
    user_owns: u32,
    #[get_copy = "pub"]
    user_bloods: u32,
    #[get_copy = "pub"]
    system_bloods: u32,
    #[get = "pub"]
    team: Option<TeamSummary>,
    #[get_copy = "pub"]
    respects: u32,
    #[get = "pub"]
    rank: RankType,
    #[get_copy = "pub"]
    rank_id: u32,
    #[get_copy = "pub"]
    current_rank_progress: u32,
    #[get = "pub"]
    next_rank: Option<RankType>,
    #[get_copy = "pub"]
    next_rank_points: Option<f32>,
    #[get_copy = "pub"]
    rank_ownership: u32,
    #[get_copy = "pub"]
    ranking: u32,
    #[get = "pub"]
    avatar: String,
    #[get = "pub"]
    timezone: String,
    #[serde(rename = "isVip")]
    #[get_copy = "pub"]
    is_vip: bool,
    #[serde(rename = "isDedicatedVip")]
    #[get_copy = "pub"]
    is_dedicated_vip: bool,
    #[get_copy = "pub"]
    public: bool,
    #[get = "pub"]
    country_name: String,
    #[get = "pub"]
    country_code: String,
    #[get_copy = "pub"]
    points: u32,
    #[get = "pub"]
    university: Option<UniversitySummary>,
    #[get = "pub"]
    university_name: Option<String>,
    #[get = "pub"]
    description: Option<String>,
    #[get = "pub"]
    github: Option<String>,
    #[get = "pub"]
    linkedin: Option<String>,
    #[get = "pub"]
    twitter: Option<String>,
    #[get = "pub"]
    website: Option<String>,
    #[serde(rename = "isRespected")]
    #[get_copy = "pub"]
    is_respected: bool,
    #[serde(rename = "isFollowed")]
    #[get_copy = "pub"]
    is_followed: bool,
}

#[derive(Deserialize, Debug, Getters, CopyGetters)]
pub struct TeamSummary {
    #[get_copy = "pub"]
    id: u32,
    #[get = "pub"]
    name: String,
    #[get_copy = "pub"]
    ranking: u32,
    #[get = "pub"]
    avatar: String,
}

#[derive(Deserialize, Debug, Getters, CopyGetters)]
pub struct UniversitySummary {
    #[get_copy = "pub"]
    id: u32,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    logo_thumb_url: String,
    #[get_copy = "pub"]
    rank: u32,
}

/// A summary version of [`User`] used for displaying details of content authors
#[derive(Debug, Deserialize, Getters, CopyGetters)]
pub struct ContentAuthor {
    #[serde(rename = "creator_id")]
    #[get_copy = "pub"]
    id: u32,
    #[serde(rename = "creator_name")]
    #[get = "pub"]
    name: String,
    #[serde(rename = "creator_avatar")]
    #[get = "pub"]
    avatar: String,
    #[serde(rename = "isRespected")]
    #[get_copy = "pub"]
    is_respected: bool,
}

pub(crate) fn content_author_2<'de, D>(deserializer: D) -> Result<Option<ContentAuthor>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub struct ContentAuthor2 {
        #[serde(rename = "creator2_id")]
        id: u32,
        #[serde(rename = "creator2_name")]
        name: String,
        #[serde(rename = "creator2_avatar")]
        avatar: String,
        #[serde(rename = "isRespected2")]
        is_respected: bool,
    }

    let res = ContentAuthor2::deserialize(deserializer);
    if let Ok(ContentAuthor2 {
        id,
        name,
        avatar,
        is_respected,
    }) = res
    {
        Ok(Some(ContentAuthor {
            id,
            name,
            avatar,
            is_respected,
        }))
    } else {
        Ok(None)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "User `{}`", self.name)
    }
}
