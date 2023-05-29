use hoi4save::Hoi4Date;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Hoi4Difficulty {
    Easy,
    Normal,
    Hard,
    Expert,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Hoi4Boolean {
    Yes,
    No,
}

impl From<Hoi4Boolean> for bool {
    fn from(b: Hoi4Boolean) -> bool {
        match b {
            Hoi4Boolean::Yes => true,
            Hoi4Boolean::No => false,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SavedEventTarget {
    pub country: Option<String>,
    pub state: Option<i32>,
    pub name: String,
}

#[derive(Debug)]
struct SavedEventTargets(Vec<SavedEventTarget>);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RailwatGunIndex {
    pub id: i32
}

#[derive(Deserialize, Debug, Clone)]
pub struct Hoi4Save {
    pub player: String,
    pub date: Hoi4Date,
    pub difficulty: Hoi4Difficulty,
    pub version: String,
    pub ironman: Option<String>,
    pub tutorial: Hoi4Boolean,
    pub save_version: i32,
    pub dlcs: i32,
    pub mods: Vec<String>,
    pub achievement: Vec<i32>,
    pub session: i32,
    pub speed: i32,
    pub game_unique_seed: i32,
    pub game_unique_id: String,
    pub multiplayer_random_seed: i32,
    pub multiplayer_random_count: i32,
    pub debug_current_ref_id: i32,
    pub unit: i32,
    pub order_index: i32,
    pub front_index: i32,
    pub theatre_index: i32,
    pub theater_group_index: i32,
    pub military_deployment_line_index: i32,
    pub military_deployment_conveyor_index: i32,
    pub equipment_variant_index: i32,
    pub country_leader_index: i32,
    pub navy_id: i32,
    pub land_combat_id: i32,
    pub railway_gun_index: RailwatGunIndex,
    pub saved_event_target: SavedEventTarget
}