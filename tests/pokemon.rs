use std::net::TcpListener;

use pokedex::{
    funtranslations_client::FunTranslationsClient, pokeapi_client::PokeapiClient, startup::run,
};
use serde_json::Value;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

pub struct TestApp {
    pub address: String,
    pub pokeapi_server: MockServer,
    pub funtranslations_server: MockServer,
}

impl TestApp {
    pub async fn get_pokemon_info(&self, name: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!("{}/pokemon/{}", &self.address, name))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn get_pokemon_info_translated(&self, name: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!("{}/translated/{}", &self.address, name))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    let pokeapi_server = MockServer::start().await;
    let funtranslations_server = MockServer::start().await;

    let pokmonapi_base_url = format!("{}/api/v2/pokemon-species/", pokeapi_server.uri());
    let funtranslations_base_url = format!("{}/translate/", funtranslations_server.uri());

    let timeout = std::time::Duration::from_secs(2);

    let pokeapi_client = PokeapiClient::new(pokmonapi_base_url, timeout);
    let funtranslations_client = FunTranslationsClient::new(funtranslations_base_url, timeout);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server =
        run(pokeapi_client, funtranslations_client, listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        pokeapi_server,
        funtranslations_server,
    }
}

#[actix_rt::test]
async fn calls_pokeapi_mockserver_to_get_info() {
    let app = spawn_app().await;
    Mock::given(path("/api/v2/pokemon-species/test"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.pokeapi_server)
        .await;
    app.get_pokemon_info("test").await;
}
#[actix_rt::test]
async fn pokemon_endpoint_returns_404_if_pokemon_not_found() {
    let app = spawn_app().await;
    Mock::given(path("/api/v2/pokemon-species/not_a_pokemon"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&app.pokeapi_server)
        .await;
    let res = app.get_pokemon_info("not_a_pokemon").await;
    assert_eq!(404, res.status().as_u16());
}
#[actix_rt::test]
async fn calls_pokeapi_and_funtranslations_mockserver_to_translate() {
    let app = spawn_app().await;

    let data = r#"
    {
        "name": "ditto",
        "flavor_text_entries": [
            {
                "flavor_text": "Capable of copying\nan enemy's genetic\ncode to instantly\ftransform itself\ninto a duplicate\nof the enemy.",
                "language": {
                    "name": "en"
                },
                "version": {
                    "name": "red"
                }
            }
        ],
        "habitat": {
            "name": "urban"
        },
        "is_legendary": false
    }"#;
    let v: Value = serde_json::from_str(data).unwrap();
    let response_template = ResponseTemplate::new(200).set_body_json(v);
    Mock::given(path("/api/v2/pokemon-species/ditto"))
        .and(method("GET"))
        .respond_with(response_template)
        .expect(1)
        .mount(&app.pokeapi_server)
        .await;

    Mock::given(path("translate/shakespeare"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.funtranslations_server)
        .await;
    app.get_pokemon_info_translated("ditto").await;
}

#[actix_rt::test]
async fn funtranslations_server_returns_429_too_many_requests() {
    let app = spawn_app().await;

    let data = r#"
    {
        "name": "ditto",
        "flavor_text_entries": [
            {
                "flavor_text": "Capable of copying\nan enemy's genetic\ncode to instantly\ftransform itself\ninto a duplicate\nof the enemy.",
                "language": {
                    "name": "en"
                },
                "version": {
                    "name": "red"
                }
            }
        ],
        "habitat": {
            "name": "urban"
        },
        "is_legendary": false
    }"#;
    let v: Value = serde_json::from_str(data).unwrap();
    let response_template = ResponseTemplate::new(200).set_body_json(v);
    Mock::given(path("/api/v2/pokemon-species/ditto"))
        .and(method("GET"))
        .respond_with(response_template)
        .expect(1)
        .mount(&app.pokeapi_server)
        .await;

    Mock::given(path("translate/shakespeare"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(429))
        .expect(1)
        .mount(&app.funtranslations_server)
        .await;
    app.get_pokemon_info_translated("ditto").await;
}
