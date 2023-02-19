use warp::Filter;
use warp::Reply;

#[shuttle_service::main]
async fn warp() -> shuttle_service::ShuttleWarp<(impl Reply,)> {
    Ok(emoji_auth::server().boxed())
}
