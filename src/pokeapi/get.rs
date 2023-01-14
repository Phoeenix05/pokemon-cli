use super::interface::PokeapiRes;

pub async fn get_by_name(name: String) -> Result<PokeapiRes, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{name}");
    
    let pokemon: PokeapiRes = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    Ok(pokemon)
}
