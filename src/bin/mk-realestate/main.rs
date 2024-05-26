extern crate log;
extern crate pretty_env_logger;
use log::{debug, info};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct ListingsBody {
    listings: Vec<Listing>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Listing {
    address: String,
    cover_image: String,
    price: i32,
    description: String,
    listing_status: String,
    listing_number: String,
    bedrooms: i8,
    square_feet: i32,
    bathrooms: i8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let mls_listings_url = env!("MLS_LISTINGS_URL");

    if mls_listings_url.is_empty() {
        panic!("`MLS_LISTINGS_URL`: {}", mls_listings_url);
    }

    let mls_listings_url =
        Url::parse(mls_listings_url).expect("Unable to parse `MLS_LISTINGS_URL` as valid url");

    let mls_listings_resp = reqwest::get(mls_listings_url).await?.text().await?;

    let document = Html::parse_document(&mls_listings_resp);

    let root_selector =
        Selector::parse("div.container").expect("Unable to parse root container div");
    let root = Html::parse_fragment(
        &document
            .select(&root_selector)
            .next()
            .expect("Root div not found")
            .inner_html(),
    );

    debug!("Root div: {:?}", root);

    let listings_selector =
        Selector::parse("div.row.searchResult").expect("Unable to parse listings");
    let listings: Vec<_> = document.select(&listings_selector).collect();

    info!("Amount of listings: {}", &listings.len());

    let mut listings_results = Vec::new();

    for listing in listings {
        let listing = Html::parse_fragment(&listing.inner_html());

        let cover_image_selector =
            Selector::parse("a.listing-image img").expect("Unable to parse cover image");
        let cover_image = listing
            .select(&cover_image_selector)
            .next()
            .expect("Cover image not found")
            .value()
            .attr("src")
            .expect("Cover image src not found");

        let address_selector = Selector::parse("h4.address").expect("Unable to parse address");
        let address = listing
            .select(&address_selector)
            .next()
            .expect("Address not found")
            .text()
            .collect::<String>();

        let price_selector = Selector::parse("h4.rapIDXSearchResultsPriceTop strong")
            .expect("Unable to parse price");
        let price = listing
            .select(&price_selector)
            .next()
            .expect("Price not found")
            .text()
            .collect::<String>()
            // Remove commas and dollar sign from price
            .replace(",", "")
            .replace("$", "")
            .parse::<i32>()
            .expect("Could not parse price as an i32 integer");

        let listing_number_selector =
            Selector::parse("span.listingNum").expect("Unable to parse listing number");
        let listing_number = listing
            .select(&listing_number_selector)
            .next()
            .expect("Listing number not found")
            .text()
            .collect::<String>();

        let listing_status_selector =
            Selector::parse("span.listingStatus em").expect("Unable to parse listing status");
        let listing_status = listing
            .select(&listing_status_selector)
            .next()
            .expect("Listing status not found")
            .text()
            .collect::<String>();

        let specs_selector =
            Selector::parse("div.specTableResults").expect("Unable to parse specs");
        let specs = listing
            .select(&specs_selector)
            .next()
            .expect("Specs not found");

        let bedrooms_selector =
            Selector::parse("div.listing-info.listingBeds div").expect("Unable to parse bedrooms");
        let bedrooms = specs
            .select(&bedrooms_selector)
            .next()
            .expect("Specs bedroom(s) wrapper not found")
            .first_child()
            .expect("Bedroom(s) wrapper not found")
            .value()
            .as_text()
            .expect("Bedroom(s) text not found")
            .to_string()
            .parse::<i8>()
            .expect("Could not parse bedrooms as an i8 integer");

        let bathrooms_selector = Selector::parse("div.listing-info.listingBaths div")
            .expect("Unable to parse bathrooms");
        let bathrooms = specs
            .select(&bathrooms_selector)
            .next()
            .expect("Specs bathroom(s) wrapper not found")
            .first_child()
            .expect("Bathroom(s) wrapper not found")
            .value()
            .as_text()
            .expect("Bathroom(s) text not found")
            .to_string()
            //The bathroom string contains the count ex: 2 and half baths ex: (2, 0)
            //We will only use the count of full bathrooms
            .split(" ")
            .collect::<Vec<&str>>()
            .first()
            .expect("Was not able to get specific bathroom count")
            .parse::<i8>()
            .expect("Could not parse bathrooms as an i8 integer");

        let square_feet_selector = Selector::parse("div.listing-info.listingSqFt div")
            .expect("Unable to parse square feet");
        let square_feet = specs
            .select(&square_feet_selector)
            .next()
            .expect("Specs square feet wrapper not found")
            .first_child()
            .expect("Square feet wrapper not found")
            .value()
            .as_text()
            .expect("Square feet text not found")
            .to_string()
            .replace(",", "")
            .parse::<i32>()
            .expect("Could not parse square feet as an i32 integer");

        let description_selector =
            Selector::parse("div.remarks div").expect("Unable to parse description");
        let description = listing
            .select(&description_selector)
            .next()
            .expect("Description not found")
            .text()
            .collect::<String>();

        info!(
            "Address: {}, \n Cover image: {:?} \n Price: {} \n Listing number: {} \n Listing status: {} \n Bedrooms: {} \n Bathrooms: {} \n Square feet: {} \n Description: {}",
            &address, &cover_image, price, listing_number, listing_status, bedrooms, bathrooms, square_feet, description.trim()
        );

        let listing = Listing {
            address,
            cover_image: cover_image.to_string(),
            price,
            listing_number,
            listing_status,
            bedrooms,
            bathrooms,
            square_feet,
            description,
        };

        listings_results.push(listing);
    }

    let listings_api_url = env!("MK_REALESTATE_LISTINGS_API_URL");
    let listings_api_token = env!("MK_REALESTATE_LISTINGS_API_TOKEN");

    let listings_api_url =
        Url::parse(listings_api_url).expect("Unable to parse `LISTINGS_API_URL` as valid url");

    let client = reqwest::Client::new();

    let body = ListingsBody {
        listings: listings_results,
    };

    client
        .post(listings_api_url)
        .bearer_auth(listings_api_token)
        .json(&body)
        .send()
        .await?;

    Ok(())
}
