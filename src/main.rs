use std::net::TcpListener;
use feeder::configuration::get_configuration;
use feeder::poster_client::PosterClient;
use feeder::startup::run;
use feeder::user_client::UserClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration()
        .expect("Failed to read configuration.");

    let poster_client = PosterClient::new(
        configuration.poster_client.base_url,
        configuration.poster_client.path
    );

    let user_client = UserClient::new(
        configuration.user_client.base_url,
        configuration.user_client.count_path,
        configuration.user_client.fetch_path
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind address");
    run(listener, user_client, poster_client)?.await
}