pub mod video;
use reqwest;
use std::env;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use serde_json::Result;
use url::UrlQuery;
use video::SanityVideo;

fn main() {
    pretty_env_logger::init();

    let sanity_project_id = env::var("SANITY_PROJECT_ID").expect("Missing SANITY_PROJECT_ID");
    let sanity_dataset = env::var("SANITY_DATASET").expect("Missing SANITY_DATASET");
    let sanity_api_token = env::var("SANITY_API_TOKEN").expect("Missing SANITY_API_TOKEN");

    //Get all videos from Sanity

    let client = reqwest::blocking::Client::new();

    //Sanity Groq query
    let query = "*[_type == video]{
     videoDuration,
       views,
        url,
      _id
}";

    let sanity_video_res = client
        .get(format!(
            "https://{}.api.sanity.io/v2021-06-07/data/query/{}?query={}",
            sanity_project_id, sanity_dataset, query
        ))
        .bearer_auth(sanity_api_token.clone())
        .send()
        .unwrap();

    match sanity_video_res.status() {
        reqwest::StatusCode::OK => {
            // let sanity_videos: Result<Vec<SanityVideo>> =
            //     serde_json::from_str(&sanity_video_res.text().unwrap());

            // if let Ok(sanity_videos) = sanity_videos {
            //     info!("Sanity videos: {:?}", sanity_videos);
            // } else {
            //     error!("Error parsing Sanity video: {:?}", sanity_videos);
            // }
            info!("Sanity video res: {:?}", sanity_video_res.text());
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            error!("Unauthorized! Response text:{:?}", sanity_video_res.text());
        }
        _ => {
            error!(
                "Unexpected Error: Response text:{:?}",
                sanity_video_res.text()
            );
        }
    }
}
