use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SanityVideo {
    url: String,
    views: i32,
    #[serde(rename(deserialize = "videoDuration"))]
    video_duration: i32,
    _id: String,
}
