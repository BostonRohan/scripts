use axum::{http::StatusCode, response::IntoResponse};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Claims {
    data: String,
}

#[derive(Debug, Deserialize)]
struct Event {
    eventType: String,
    data: String,
}

pub async fn webhook(body: String) -> impl IntoResponse {
    let pub_key = env!("WIX_PUBLIC_KEY");

    let validation = Validation::new(jsonwebtoken::Algorithm::RS256);

    let pub_key = match DecodingKey::from_rsa_pem(pub_key.as_bytes()) {
        Ok(k) => k,
        Err(err) => {
            let error = format!("Error decoding public key: {:?}", err);
            eprintln!("{}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response();
        }
    };

    let raw_payload = match decode::<Claims>(&body, &pub_key, &validation) {
        Ok(c) => c,
        Err(err) => {
            let error = format!("Error decoding raw payload: {:?}", err);
            eprintln!("{}", error);
            return (StatusCode::BAD_REQUEST, error).into_response();
        }
    };

    let event: Event = match serde_json::from_str(&raw_payload.claims.data) {
        Ok(event) => event,
        Err(err) => {
            let error = format!("Error parsing event: {:?}", err);
            error!("{:?}", error);
            return (StatusCode::BAD_REQUEST, error).into_response();
        }
    };

    match event.eventType.as_str() {
        "AppInstalled" => {
            info!("AppInstalled event received with data: {}", event.data);
            // handle your event here
        }
        _ => {
            warn!("Received unknown event type: {}", event.eventType);
        }
    }

    StatusCode::OK.into_response()
}
