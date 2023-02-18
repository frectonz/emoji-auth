use unicode_segmentation::UnicodeSegmentation;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let create_todo = warp::path("greeting")
        .and(warp::get())
        .and(warp::header("authorization"))
        .map(|auth: String| {
            let mut auth = auth.split_whitespace();

            let (Some("Emoji"), Some(url_encoded_auth_token)) = (auth.next(), auth.next()) else {
                return "Invalid authorization header".to_string();
            };

            let Ok(auth_token) = urlencoding::decode(url_encoded_auth_token) else {
                return "Token is not url encoded".to_string();
            };

            let emojis = auth_token
                .graphemes(true)
                .filter_map(emojis::get)
                .map(|e| e.as_str())
                .collect::<Vec<_>>();

            if emojis.len() == 4 {
                format!("Hello, {}!", emojis.join(""))
            } else {
                "You must provide exactly 4 emojis".to_string()
            }
        });

    warp::serve(create_todo).run(([127, 0, 0, 1], 3030)).await;
}
