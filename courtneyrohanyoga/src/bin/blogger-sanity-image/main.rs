mod post_images_to_sanity;
use crate::post_images_to_sanity::post_images_to_sanity;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct BloggerReplies {
    #[serde(rename(deserialize = "totalItems"))]
    total_items: String,
    #[serde(rename(deserialize = "selfLink"))]
    self_link: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct BloggerImage {
    url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct BloggerAuthor {
    id: String,
    #[serde(rename(deserialize = "displayName"))]
    display_name: String,
    url: String,
    image: BloggerImage,
}
#[derive(Serialize, Deserialize, Debug)]
struct BloggerPostRes {
    kind: String,
    items: Vec<BloggerPost>,
}
#[derive(Serialize, Deserialize, Debug)]
struct BloggerBlogId {
    id: String,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct BloggerPost {
    kind: String,
    id: String,
    blog: BloggerBlogId,
    published: String,
    updated: String,
    url: String,
    #[serde(rename(deserialize = "selfLink"))]
    self_link: String,
    title: String,
    content: String,
    author: BloggerAuthor,
    replies: BloggerReplies,
}

impl BloggerPost {
    fn post_images_to_sanity(&self) {
        post_images_to_sanity(&self)
    }
}
fn main() {
    let blogger_id = env::var("BLOGGER_ID").expect("Missing BLOGGER_ID");
    let blogger_api_key = env::var("BLOGGER_API_KEY").expect("Missing BLOGGER_API_KEY");

    let content = reqwest::blocking::get(format!(
        "https://www.googleapis.com/blogger/v3/blogs/{}/posts?key={}&maxResults=500",
        blogger_id, blogger_api_key
    ))
    .unwrap();

    let posts_list_res: Result<BloggerPostRes> = serde_json::from_str(&content.text().unwrap());

    match posts_list_res {
        Ok(result) => {
            for post in result.items.iter() {
                post.post_images_to_sanity()
            }
        }
        Err(err) => panic!("{}", err),
    }
}
