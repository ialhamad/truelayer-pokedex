use actix_web::{web, HttpResponse};
use reqwest::StatusCode;

use crate::{
    funtranslations_client::{FunTranslationsClient, TranslationType},
    models::Pokemon,
    pokeapi_client::PokeapiClient,
};

pub async fn pokemon_data(
    pokeapi_client: web::Data<PokeapiClient>,
    path: web::Path<String>,
) -> HttpResponse {
    let name = path.into_inner();
    let json = match pokeapi_client.get_pokemon_by_name(name).await {
        Ok(pokemon) => pokemon,
        Err(e) => return handle_api_err(e),
    };
    HttpResponse::build(StatusCode::OK).json(json)
}
pub async fn pokemon_data_translated(
    funtranslations_client: web::Data<FunTranslationsClient>,
    pokeapi_client: web::Data<PokeapiClient>,
    path: web::Path<String>,
) -> HttpResponse {
    let name = path.into_inner();
    match pokeapi_client.get_pokemon_by_name(name).await {
        Ok(pokemon) => {
            let pokemon = translate_pokemon(pokemon, &funtranslations_client).await;
            HttpResponse::build(StatusCode::OK).json(pokemon)
        }
        Err(e) => handle_api_err(e),
    }
}

async fn translate_pokemon(
    mut pokemon: Pokemon,
    funtranslations_client: &FunTranslationsClient,
) -> Pokemon {
    // match funtranslations_client
    //     .translate(pokemon.get_description(), translate_into(&pokemon))
    //     .await
    // {
    //     Ok(translated_desc) => {
    //         pokemon.set_description(&translated_desc);
    //     }
    //     Err(e) => {
    //         if let Some(status) = e.status() {
    //             return match status.as_u16() {
    //                 429 => Ok(pokemon),
    //                 _ => Err(e),
    //             };
    //         }
    //     }
    // }

    if let Ok(translated_desc) = funtranslations_client
        .translate(pokemon.get_description(), translate_into(&pokemon))
        .await
    {
        pokemon.set_description(&translated_desc);
    }

    // Always returns the pokemon even if the translation fails.
    //
    pokemon
}

fn translate_into(pokemon: &Pokemon) -> TranslationType {
    if pokemon.is_cave_habitat() || pokemon.is_legendary {
        TranslationType::Yoda
    } else {
        TranslationType::Shakespeare
    }
}

fn handle_api_err(err: reqwest::Error) -> HttpResponse {
    if let Some(status) = err.status() {
        HttpResponse::new(StatusCode::from_u16(status.as_u16()).unwrap())
    } else {
        HttpResponse::ServiceUnavailable().finish()
    }
}
