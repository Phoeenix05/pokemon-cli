use super::interface::PokeapiRes;

/// We're using the `reqwest` crate to make a GET request to the PokeAPI, and then we're parsing the
/// response into a `PokeapiRes` struct
/// 
/// Arguments:
/// 
/// * `name`: The name of the pokemon we want to get.
/// 
/// Returns:
/// 
/// A Result<PokeapiRes, reqwest::Error>
pub async fn get_pokemon(mut name: String) -> Result<PokeapiRes, reqwest::Error> {
    name = name.to_lowercase();
    let url = format!("https://pokeapi.co/api/v2/pokemon/{name}");

    let pokemon: PokeapiRes = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json() // Convert response to json
        .await?;

    Ok(pokemon)
}
