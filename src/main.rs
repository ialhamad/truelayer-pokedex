use std::net::TcpListener;

use log::info;
use pokedex::{
    funtranslations_client::FunTranslationsClient, pokeapi_client::PokeapiClient, startup::run,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let pokmonapi_base_url = "https://pokeapi.co/api/v2/pokemon-species/".into();
    let funtranslations_base_url = "https://api.funtranslations.com/translate/".into();

    let timeout = std::time::Duration::from_secs(2);
    let pokeapi_client = PokeapiClient::new(pokmonapi_base_url, timeout);
    let funtranslations_client = FunTranslationsClient::new(funtranslations_base_url, timeout);
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(&address)?;
    let server = run(pokeapi_client, funtranslations_client, listener)?;
    info!("service is running on http://{}", &address);
    server.await
}
