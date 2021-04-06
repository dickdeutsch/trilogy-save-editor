use anyhow::Result;

use crate::serializer::{SaveCursor, SaveData};

use super::player::{Power, Weapon, WeaponLoadout, WeaponMod};

#[derive(SaveData, Debug)]
pub(super) struct Henchman {
    tag: String,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: String,
    weapon_mods: Vec<WeaponMod>,
    grenades: i32,
    weapons: Vec<Weapon>,
}
