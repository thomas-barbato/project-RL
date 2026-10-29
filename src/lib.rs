//! Headless simulation core for Project RL.
//!
//! This library deliberately has no dependency on Macroquad types. The binary
//! and future presentation layers consume its commands, state and events.

#[cfg(debug_assertions)]
pub mod action_profile;
pub mod agent_class;
pub mod ai;
pub mod asset_cache;
pub mod character_class;
pub mod combat;
pub mod companion;
pub mod content;
pub mod data_paths;
pub mod drone;
pub mod effects;
pub mod electronic_warfare;
pub mod engineering;
pub mod entity;
pub mod explosive;
pub mod facility;
pub mod game;
pub mod intrusion;
pub mod item;
pub mod localization;
pub mod loading_profile;
pub mod loot;
pub mod monster_art;
pub mod monster_modifiers;
pub mod monster_palettes;
pub mod presentation;
pub mod progression;
pub mod reaction;
pub mod resources;
pub mod skills;
pub mod social;
pub mod stats;
pub mod status;
pub mod stealth;
pub mod time;
pub mod weapon;
pub mod world;
