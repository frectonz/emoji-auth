#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    warp::serve(emoji_auth::server())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
