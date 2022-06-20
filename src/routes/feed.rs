use actix_web::{HttpResponse, Responder, web};
use crate::poster_client::PosterClient;
use crate::user_client::UserClient;
use tracing::{info, instrument};

#[derive(serde::Deserialize)]
pub struct FeedRequest {
    username: String,
    page: u64
}

#[instrument(
    name = "Getting feed latest posts",
    skip(req, user_client, poster_client)
)]
pub async fn create_feed(
    req: web::Json<FeedRequest>,
    user_client: web::Data<UserClient>,
    poster_client: web::Data<PosterClient>
) -> impl Responder {

    let followings_count = match user_client.get_following_count(&req.username).await {
        Ok(count) => count,
        Err(_) => {
            info!("Unable to fetch following count");
            return HttpResponse::InternalServerError().finish()
        }
    };

    let followings = match user_client.get_followees(&req.username, followings_count).await {
        Ok(list) => list,
        Err(_) => {
            info!("Unable to fetch followings");
            return HttpResponse::InternalServerError().finish()
        }
    };

    let latest_posts = match poster_client.get_posts(followings, req.page).await {
        Ok(posts) => posts,
        Err(_) => {
            info!("Unable to fetch posts");
            return HttpResponse::InternalServerError().finish()
        }
    };

    HttpResponse::Ok().json(&latest_posts)
}
