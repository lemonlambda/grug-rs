use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModAPI {
    pub entities: LinkedHashMap<String, Entity>,
    pub game_functions: LinkedHashMap<String, GameFunction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub description: String,
    pub on_functions: LinkedHashMap<String, GameFunction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameFunction {
    pub description: String,
    #[serde(default)]
    pub arguments: Vec<Argument>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Argument {
    pub name: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_: String,
}
