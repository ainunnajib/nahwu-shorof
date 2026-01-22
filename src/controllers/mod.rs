pub mod home;
pub mod learn;
pub mod play;
pub mod profile;
pub mod api;
mod data;

use loco_rs::prelude::*;

/// Create a seed task for database seeding
pub fn seed_task() -> Task {
    Task::new("seed", "Seed the database with initial data")
}
