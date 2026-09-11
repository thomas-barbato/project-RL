//! Headless simulation core for Project RL.
//!
//! This library deliberately has no dependency on Macroquad types. The binary
//! and future presentation layers consume its commands, state and events.

pub mod ai;
pub mod combat;
pub mod content;
pub mod effects;
pub mod entity;
pub mod facility;
pub mod game;
pub mod item;
pub mod localization;
pub mod loot;
pub mod presentation;
pub mod progression;
pub mod resources;
pub mod skills;
pub mod social;
pub mod stats;
pub mod status;
pub mod weapon;
pub mod world;
