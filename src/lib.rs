//! Headless simulation core for Project RL.
//!
//! This library deliberately has no dependency on Macroquad types. The binary
//! and future presentation layers consume its commands, state and events.

pub mod ai;
pub mod combat;
pub mod content;
pub mod effects;
pub mod entity;
pub mod game;
pub mod progression;
pub mod status;
pub mod weapon;
pub mod world;
