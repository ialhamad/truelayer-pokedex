use reqwest::Client;

use crate::models::Pokemon;

pub struct PokeapiClient {
    http_client: Client,
    base_url: String,
}

impl PokeapiClient {
    pub fn new(base_url: String, timeout: std::time::Duration) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            base_url,
        }
    }

    pub async fn get_pokemon_by_name(&self, name: String) -> Result<Pokemon, reqwest::Error> {
        let url = format!("{}{}", self.base_url, &name);
        let res = self.http_client.get(url).send().await?;
        res.json::<Pokemon>().await
    }
}

