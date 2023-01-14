use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PokeapiRes {
    pub abilities: Vec<Ability>,
    pub base_experience: i64,
    pub forms: Vec<Species>,
    pub game_indices: Vec<GameIndex>,
    pub height: i64,
    pub held_items: Vec<HeldItem>,
    pub id: i64,
    pub is_default: bool,
    pub location_area_encounters: String,
    pub moves: Vec<Move>,
    pub name: String,
    pub order: i64,
    pub past_types: Vec<Option<serde_json::Value>>,
    pub species: Species,
    pub sprites: Box<Sprites>,
    pub stats: Vec<Stat>,
    pub types: Vec<Type>,
    pub weight: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub ability: Species,
    pub is_hidden: bool,
    pub slot: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Species {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameIndex {
    pub game_index: i64,
    pub version: Species,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeldItem {
    pub item: Species,
    pub version_details: Vec<VersionDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionDetail {
    pub rarity: i64,
    pub version: Species,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    #[serde(rename = "move")]
    pub move_move: Species,
    pub version_group_details: Vec<VersionGroupDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionGroupDetail {
    pub level_learned_at: i64,
    pub move_learn_method: Species,
    pub version_group: Species,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationV {
    #[serde(rename = "black-white")]
    pub black_white: Box<Sprites>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationIv {
    #[serde(rename = "diamond-pearl")]
    pub diamond_pearl: Box<Sprites>,
    #[serde(rename = "heartgold-soulsilver")]
    pub heartgold_soulsilver: Box<Sprites>,
    pub platinum: Box<Sprites>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Versions {
    #[serde(rename = "generation-i")]
    pub generation_i: GenerationI,
    #[serde(rename = "generation-ii")]
    pub generation_ii: GenerationIi,
    #[serde(rename = "generation-iii")]
    pub generation_iii: GenerationIii,
    #[serde(rename = "generation-iv")]
    pub generation_iv: GenerationIv,
    #[serde(rename = "generation-v")]
    pub generation_v: GenerationV,
    #[serde(rename = "generation-vi")]
    pub generation_vi: HashMap<String, Home>,
    #[serde(rename = "generation-vii")]
    pub generation_vii: GenerationVii,
    #[serde(rename = "generation-viii")]
    pub generation_viii: GenerationViii,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sprites {
    pub back_default: String,
    pub back_female: Option<serde_json::Value>,
    pub back_shiny: String,
    pub back_shiny_female: Option<serde_json::Value>,
    pub front_default: String,
    pub front_female: Option<serde_json::Value>,
    pub front_shiny: String,
    pub front_shiny_female: Option<serde_json::Value>,
    pub other: Option<Other>,
    pub versions: Option<Versions>,
    pub animated: Option<Box<Sprites>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationI {
    #[serde(rename = "red-blue")]
    pub red_blue: RedBlue,
    pub yellow: RedBlue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedBlue {
    pub back_default: String,
    pub back_gray: String,
    pub back_transparent: String,
    pub front_default: String,
    pub front_gray: String,
    pub front_transparent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationIi {
    pub crystal: Crystal,
    pub gold: Gold,
    pub silver: Gold,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crystal {
    pub back_default: String,
    pub back_shiny: String,
    pub back_shiny_transparent: String,
    pub back_transparent: String,
    pub front_default: String,
    pub front_shiny: String,
    pub front_shiny_transparent: String,
    pub front_transparent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gold {
    pub back_default: String,
    pub back_shiny: String,
    pub front_default: String,
    pub front_shiny: String,
    pub front_transparent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationIii {
    pub emerald: OfficialArtwork,
    #[serde(rename = "firered-leafgreen")]
    pub firered_leafgreen: Gold,
    #[serde(rename = "ruby-sapphire")]
    pub ruby_sapphire: Gold,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfficialArtwork {
    pub front_default: String,
    pub front_shiny: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Home {
    pub front_default: String,
    pub front_female: Option<serde_json::Value>,
    pub front_shiny: String,
    pub front_shiny_female: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationVii {
    pub icons: DreamWorld,
    #[serde(rename = "ultra-sun-ultra-moon")]
    pub ultra_sun_ultra_moon: Home,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DreamWorld {
    pub front_default: String,
    pub front_female: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationViii {
    pub icons: DreamWorld,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Other {
    pub dream_world: DreamWorld,
    pub home: Home,
    #[serde(rename = "official-artwork")]
    pub official_artwork: OfficialArtwork,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub base_stat: i64,
    pub effort: i64,
    pub stat: Species,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    pub slot: i64,
    #[serde(rename = "type")]
    pub type_type: Species,
}
