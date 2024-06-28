use log::info;
use reqwest::{self, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BlogPost {
    id: String,
    title: String,
    excerpt: String,
    first_published_date: String,
    last_published_date: String,
    slug: String,
    featured: bool,
    pinned: bool,
    category_ids: Vec<String>,
    member_id: String,
    hashtags: Vec<String>,
    commenting_enabled: bool,
    minutes_to_read: i32,
    tag_ids: Vec<String>,
    rich_content: Option<Value>,
    cover_media: CoverMedia,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlogPosts {
    posts: Vec<BlogPost>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CoverMedia {
    image: Image,
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    id: String,
    url: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let wix_api_token = env!("WIX_API_TOKEN");
    let wix_account_id = env!("WIX_ACCOUNT_ID");
    let wix_site_id = env!("WIX_SITE_ID");

    let client = reqwest::Client::new();

    let blog_posts_res = client
        .get("https://www.wixapis.com/v3/posts?fieldsets=RICH_CONTENT")
        .header(AUTHORIZATION, wix_api_token)
        .header("wix-account-id", wix_account_id)
        .header("wix-site-id", wix_site_id)
        .send()
        .await
        .expect("Failed to get blog posts");

    let blog_posts: BlogPosts = serde_json::from_str(
        &blog_posts_res
            .text()
            .await
            .expect("Failed to get blog posts response text"),
    )
    .expect("Failed to parse blog posts response");

    info!("blog posts: {:?}", blog_posts);
}
