use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "progress")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub topic_id: i32,
    pub stars_earned: i32, // 0, 1, 2, or 3
    pub best_score: i32,
    pub attempts: i32,
    pub completed_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::topics::Entity",
        from = "Column::TopicId",
        to = "super::topics::Column::Id"
    )]
    Topic,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::topics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Topic.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Calculate stars from score percentage
    pub fn calculate_stars(score: i32, max_score: i32) -> i32 {
        if max_score == 0 {
            return 0;
        }
        let percentage = (score as f32 / max_score as f32) * 100.0;
        if percentage >= 90.0 {
            3
        } else if percentage >= 70.0 {
            2
        } else if percentage >= 50.0 {
            1
        } else {
            0
        }
    }
}
