use std::convert::Infallible;

use unicode_segmentation::UnicodeSegmentation;
use warp::{Filter, Rejection, Reply};

pub fn server() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let greeting = warp::path("greeting")
        .and(warp::get())
        .and(warp::header("authorization"))
        .and_then(handler);

    return greeting;
}

async fn handler(auth: String) -> Result<impl warp::Reply, Infallible> {
    let mut auth = auth.split_whitespace();

    let (Some("Emoji"), Some(url_encoded_auth_token)) = (auth.next(), auth.next()) else {
                return Ok(warp::reply::with_status(
                    "Authorization header must be of the form 'Emoji <token>'".to_string(),
                    warp::http::StatusCode::BAD_REQUEST,
                ));
            };

    let Ok(auth_token) = urlencoding::decode(url_encoded_auth_token) else {
                return Ok(
                    warp::reply::with_status("Token must be url-encoded".to_string(), warp::http::StatusCode::BAD_REQUEST),
                )
            };

    let mut emojis = auth_token.graphemes(true).map(emojis::get);

    let all_are_emojis = emojis.all(|o| o.is_some());

    if !all_are_emojis {
        return Ok(warp::reply::with_status(
            "Token must be a sequence of emojis".to_string(),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    let emojis = emojis
        .map(|o| o.unwrap())
        .map(|e| e.as_str())
        .collect::<Vec<_>>();

    if emojis.len() != 4 {
        return Ok(warp::reply::with_status(
            "Token must be a sequence of 4 emojis".to_string(),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    Ok(warp::reply::with_status(
        format!("Hello, {}!", emojis.join(" ")),
        warp::http::StatusCode::OK,
    ))
}

#[shuttle_service::main]
async fn warp() -> shuttle_service::ShuttleWarp<(impl Reply,)> {
    Ok(server().boxed())
}
