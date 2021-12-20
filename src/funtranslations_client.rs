use reqwest::Client;
use serde::Deserialize;

pub struct FunTranslationsClient {
    http_client: Client,
    base_url: String,
}

#[derive(Deserialize)]
struct TranslationContents {
    translated: String,
}

#[derive(Deserialize)]
struct Translation {
    contents: TranslationContents,
}

pub enum TranslationType {
    Yoda,
    Shakespeare,
}

impl FunTranslationsClient {
    pub fn new(base_url: String, timeout: std::time::Duration) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            base_url,
        }
    }

    pub async fn translate(
        &self,
        text: &str,
        translation_type: TranslationType,
    ) -> Result<String, reqwest::Error> {
        let translate_into = match translation_type {
            TranslationType::Yoda => "yoda",
            TranslationType::Shakespeare => "shakespeare",
        };
        let url = format!("{}{}", self.base_url, translate_into);
        let res = self
            .http_client
            .post(url)
            .form(&[("text", text)])
            .send()
            .await?;
        let translation_res = res.json::<Translation>().await?;
        Ok(translation_res.contents.translated)
    }
}
