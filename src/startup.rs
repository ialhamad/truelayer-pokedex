use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};

use crate::{funtranslations_client::FunTranslationsClient, pokeapi_client::PokeapiClient, routes};

pub fn run(
    pokeapi_client: PokeapiClient,
    funtranslations_client: FunTranslationsClient,
    listener: TcpListener,
) -> Result<Server, std::io::Error> {
    let pokeapi_client = Data::new(pokeapi_client);
    let funtranslations_client = Data::new(funtranslations_client);
    let server = HttpServer::new(move || {
        App::new()
            .route("/pokemon/{pokemon}", web::get().to(routes::pokemon_data))
            .route(
                "/translated/{pokemon}",
                web::get().to(routes::pokemon_data_translated),
            )
            .app_data(pokeapi_client.clone())
            .app_data(funtranslations_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
