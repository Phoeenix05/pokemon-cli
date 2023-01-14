use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PokeapiRes {
    pub abilities: Option<Vec<Ability>>,
    pub base_experience: Option<i64>,
    pub forms: Option<Vec<Species>>,
    pub game_indices: Option<Vec<GameIndex>>,
    pub height: Option<i64>,
    pub held_items: Option<Vec<HeldItem>>,
    pub id: Option<i64>,
    pub is_default: Option<bool>,
    pub location_area_encounters: Option<String>,
    pub moves: Option<Vec<Move>>,
    pub name: Option<String>,
    pub order: Option<i64>,
    pub past_types: Option<Vec<Option<serde_json::Value>>>,
    pub species: Option<Species>,
    pub sprites: Option<Box<Sprites>>,
    pub stats: Option<Vec<Stat>>,
    pub types: Option<Vec<Type>>,
    pub weight: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub ability: Option<Species>,
    pub is_hidden: Option<bool>,
    pub slot: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Species {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameIndex {
    pub game_index: Option<i64>,
    pub version: Option<Species>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeldItem {
    pub item: Option<Species>,
    pub version_details: Option<Vec<VersionDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionDetail {
    pub rarity: Option<i64>,
    pub version: Option<Species>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    #[serde(rename = "move")]
    pub move_move: Option<Species>,
    pub version_group_details: Option<Vec<VersionGroupDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionGroupDetail {
    pub level_learned_at: Option<i64>,
    pub move_learn_method: Option<Species>,
    pub version_group: Option<Species>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationV {
    #[serde(rename = "black-white")]
    pub black_white: Option<Box<Sprites>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationIv {
    #[serde(rename = "diamond-pearl")]
    pub diamond_pearl: Option<Box<Sprites>>,
    #[serde(rename = "heartgold-soulsilver")]
    pub heartgold_soulsilver: Option<Box<Sprites>>,
    pub platinum: Option<Box<Sprites>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Versions {
    #[serde(rename = "generation-i")]
    pub generation_i: Option<GenerationI>,
    #[serde(rename = "generation-ii")]
    pub generation_ii: Option<GenerationIi>,
    #[serde(rename = "generation-iii")]
    pub generation_iii: Option<GenerationIii>,
    #[serde(rename = "generation-iv")]
    pub generation_iv: Option<GenerationIv>,
    #[serde(rename = "generation-v")]
    pub generation_v: Option<GenerationV>,
    #[serde(rename = "generation-vi")]
    pub generation_vi: Option<HashMap<String, Home>>,
    #[serde(rename = "generation-vii")]
    pub generation_vii: Option<GenerationVii>,
    #[serde(rename = "generation-viii")]
    pub generation_viii: Option<GenerationViii>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sprites {
    pub back_default: Option<String>,
    pub back_female: Option<Option<serde_json::Value>>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<Option<serde_json::Value>>,
    pub front_default: Option<String>,
    pub front_female: Option<Option<serde_json::Value>>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<Option<serde_json::Value>>,
    pub other: Option<Option<Other>>,
    pub versions: Option<Option<Versions>>,
    pub animated: Option<Option<Box<Sprites>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationI {
    #[serde(rename = "red-blue")]
    pub red_blue: Option<RedBlue>,
    pub yellow: Option<RedBlue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedBlue {
    pub back_default: Option<String>,
    pub back_gray: Option<String>,
    pub back_transparent: Option<String>,
    pub front_default: Option<String>,
    pub front_gray: Option<String>,
    pub front_transparent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationIi {
    pub crystal: Option<Crystal>,
    pub gold: Option<Gold>,
    pub silver: Option<Gold>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crystal {
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_transparent: Option<String>,
    pub back_transparent: Option<String>,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_transparent: Option<String>,
    pub front_transparent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gold {
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_transparent: Option<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationIii {
    pub emerald: Option<OfficialArtwork>,
    #[serde(rename = "firered-leafgreen")]
    pub firered_leafgreen: Option<Gold>,
    #[serde(rename = "ruby-sapphire")]
    pub ruby_sapphire: Option<Gold>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfficialArtwork {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Home {
    pub front_default: Option<String>,
    pub front_female: Option<Option<serde_json::Value>>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationVii {
    pub icons: Option<DreamWorld>,
    #[serde(rename = "ultra-sun-ultra-moon")]
    pub ultra_sun_ultra_moon: Option<Home>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DreamWorld {
    pub front_default: Option<String>,
    pub front_female: Option<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationViii {
    pub icons: Option<DreamWorld>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Other {
    pub dream_world: Option<DreamWorld>,
    pub home: Option<Home>,
    #[serde(rename = "official-artwork")]
    pub official_artwork: Option<OfficialArtwork>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub base_stat: Option<i64>,
    pub effort: Option<i64>,
    pub stat: Option<Species>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    pub slot: Option<i64>,
    #[serde(rename = "type")]
    pub type_type: Option<Species>,
}
