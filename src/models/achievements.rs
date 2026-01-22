use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "achievements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub r#type: String,
    pub name_id: String,
    pub name_ar: String,
    pub name_en: String,
    pub description_id: String,
    pub description_ar: String,
    pub description_en: String,
    pub icon: String,
    pub xp_reward: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_achievements::Entity")]
    UserAchievements,
}

impl Related<super::user_achievements::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAchievements.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Get name by language
    pub fn name(&self, lang: &str) -> &str {
        match lang {
            "ar" => &self.name_ar,
            "en" => &self.name_en,
            _ => &self.name_id,
        }
    }

    /// Get description by language
    pub fn description(&self, lang: &str) -> &str {
        match lang {
            "ar" => &self.description_ar,
            "en" => &self.description_en,
            _ => &self.description_id,
        }
    }
}

/// Achievement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementType {
    FirstLesson,
    Streak7,
    Streak30,
    PerfectScore,
    NahwuMaster,
    ShorofMaster,
    Level10,
    Level25,
    Level50,
}

impl AchievementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FirstLesson => "first-lesson",
            Self::Streak7 => "streak-7",
            Self::Streak30 => "streak-30",
            Self::PerfectScore => "perfect-score",
            Self::NahwuMaster => "nahwu-master",
            Self::ShorofMaster => "shorof-master",
            Self::Level10 => "level-10",
            Self::Level25 => "level-25",
            Self::Level50 => "level-50",
        }
    }
}
