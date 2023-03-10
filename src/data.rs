pub const APP_TITLE: &str = "Hackathon Shooter";

pub const PIXELS_PER_METER: f32 = 32.0;
pub const VIEW_DISTANCE: f32 = 30.0;

pub const WORLD_SIZE: f32 = 64.0;
pub const WORLD_SIZE_HALF: f32 = WORLD_SIZE / 2.0;
pub const WORLD_SIZE_VISUAL: f32 = WORLD_SIZE + VIEW_DISTANCE;

pub const LAYER_TERRAIN: f32 = 0.0;
pub const LAYER_BLUFF: f32 = 0.1;
pub const LAYER_ACTOR: f32 = 0.2;
pub const LAYER_ACTOR_PLAYER: f32 = 0.3;
pub const LAYER_PROJECTILE: f32 = 0.4;
pub const LAYER_TREE: f32 = 0.5;
