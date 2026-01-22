use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pid: Uuid,
    pub email: Option<String>,
    pub name: String,
    pub avatar: Option<String>,
    pub current_level: i32,
    pub total_xp: i32,
    pub preferred_language: String,
    pub streak: i32,
    pub last_active_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::progress::Entity")]
    Progress,
    #[sea_orm(has_many = "super::game_sessions::Entity")]
    GameSessions,
    #[sea_orm(has_many = "super::user_achievements::Entity")]
    UserAchievements,
}

impl Related<super::progress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Progress.def()
    }
}

impl Related<super::game_sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GameSessions.def()
    }
}

impl Related<super::user_achievements::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAchievements.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Calculate XP needed for next level
    pub fn xp_for_next_level(&self) -> i32 {
        self.current_level * 1000
    }

    /// Calculate XP progress within current level
    pub fn current_level_progress(&self) -> i32 {
        let previous_levels_xp: i32 = (1..self.current_level).map(|l| l * 1000).sum();
        self.total_xp - previous_levels_xp
    }

    /// Add XP and potentially level up
    pub fn add_xp(&mut self, xp: i32) -> bool {
        self.total_xp += xp;
        let mut leveled_up = false;

        while self.current_level_progress() >= self.xp_for_next_level() {
            self.current_level += 1;
            leveled_up = true;
        }

        leveled_up
    }
}
