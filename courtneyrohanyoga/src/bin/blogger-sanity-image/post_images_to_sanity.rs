use crate::BloggerPost;
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{env, io::Read};

#[derive(Serialize, Deserialize, Debug)]
struct SanityAsset {
    _id: String,
    _type: String,
    #[serde(rename(deserialize = "assetId"))]
    asset_id: String,
    path: String,
    url: String,
    #[serde(rename(deserialize = "originalFilename"))]
    original_filename: String,
    size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SanityAssetMetadata {
    dimensions: SanityAssetDimensions,
}
#[derive(Serialize, Deserialize, Debug)]
struct SanityAssetDimensions {
    height: i16,
    width: i16,
    #[serde(rename(deserialize = "aspectRatio"))]
    aspect_ratio: i8,
}

pub fn post_images_to_sanity(post: &BloggerPost) {
    let document = Html::parse_document(&post.content);
    let img_selector = Selector::parse("img").unwrap();
    let sanity_project_id = env::var("SANITY_PROJECT_ID").expect("Missing SANITY_PROJECT_ID");
    let sanity_dataset = env::var("SANITY_DATASET").expect("Missing SANITY_DATASET");
    let sanity_api_token = env::var("SANITY_API_TOKEN").expect("Missing SANITY_API_TOKEN");

    for img in document.select(&img_selector) {
        let src = img.value().attr("src").unwrap_or("unknown");

        let img_res = reqwest::blocking::get(src);

        match img_res {
            Ok(mut res) => {
                // the text after the last slash is either the file name or a weird hash that blogger created
                let mut file_name = String::from(src.split("/").last().unwrap());
                let mut img_bytes = Vec::new();
                let _ = res.read_to_end(&mut img_bytes);

                let content_type = res.headers().get(reqwest::header::CONTENT_TYPE);

                if let Some(content_type) = content_type {
                    let re = Regex::new(r"[^a-zA-Z0-9]+").unwrap();

                    file_name = format!(
                        "{}.{}",
                        //replace all non-alphanumeric with space
                        format!(
                            "{}-{}",
                            re.replace_all(&post.title.trim(), " ")
                                //trim spaces off end
                                .trim_end()
                                //replace spaces with dashes
                                .replace(" ", "-")
                                .to_string(),
                            //attach the og file hash to the new filename
                            &file_name
                        ),
                        content_type.to_str().unwrap().split("/").last().unwrap()
                    );

                    let client = reqwest::blocking::Client::new();

                    let sanity_res = client
                        .post(format!(
                            "https://{}.api.sanity.io/v2021-06-07/assets/images/{}?filename={}",
                            sanity_project_id, sanity_dataset, file_name
                        ))
                        .header(CONTENT_TYPE, content_type.clone())
                        .bearer_auth(sanity_api_token.clone())
                        .body(img_bytes)
                        .send()
                        .unwrap();

                    match sanity_res.status() {
                        reqwest::StatusCode::OK => {
                            println!("Success! Response text:{:?}", sanity_res.text(),)
                        }
                        reqwest::StatusCode::BAD_REQUEST => {
                            println!("{}", sanity_res.text().unwrap())
                        }
                        other => {
                            panic!(
                                "Uh oh! Something unexpected happened: {:?} \n Text response: {:?}",
                                other,
                                sanity_res.text()
                            );
                        }
                    }
                } else {
                    println!(
                        "Content-Type header not present in the response for image: {}",
                        src
                    );
                    break;
                }
            }
            Err(_) => println!("An error occurred fetching image: {}", src),
        }
    }
}
