use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use crate::poster_client::PosterClient;
use crate::routes::create_feed;
use crate::user_client::UserClient;

pub fn run(
    listener: TcpListener,
    user_client: UserClient,
    poster_client: PosterClient
) -> Result<Server, std::io::Error> {

    let user_client = web::Data::new(user_client);
    let poster_client = web::Data::new(poster_client);

    let server = HttpServer::new(move || {
        App::new()
            .route("/feed", web::get().to(create_feed))
            .app_data(user_client.clone())
            .app_data(poster_client.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}