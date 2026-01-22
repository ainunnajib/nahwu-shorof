use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "topics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub category: String, // nahwu, shorof
    pub order: i32,
    #[sea_orm(unique)]
    pub slug: String,
    pub title_id: String,
    pub title_ar: String,
    pub title_en: String,
    pub description_id: String,
    pub description_ar: String,
    pub description_en: String,
    pub required_level: i32,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::progress::Entity")]
    Progress,
}

impl Related<super::progress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Progress.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Get title by language
    pub fn title(&self, lang: &str) -> &str {
        match lang {
            "ar" => &self.title_ar,
            "en" => &self.title_en,
            _ => &self.title_id,
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
