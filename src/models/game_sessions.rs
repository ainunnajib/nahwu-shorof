use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game_sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub game_type: String, // word-builder, sentence-doctor, etc.
    pub topic_id: Option<String>,
    pub score: i32,
    pub max_score: i32,
    pub duration: i32, // in seconds
    pub mistakes: Json,
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
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Game types available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameType {
    WordBuilder,
    SentenceDoctor,
    GrammarMatch,
    StoryMode,
    ConjugationRace,
    IrabAnalyzer,
}

impl GameType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WordBuilder => "word-builder",
            Self::SentenceDoctor => "sentence-doctor",
            Self::GrammarMatch => "grammar-match",
            Self::StoryMode => "story-mode",
            Self::ConjugationRace => "conjugation-race",
            Self::IrabAnalyzer => "irab-analyzer",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "word-builder" => Some(Self::WordBuilder),
            "sentence-doctor" => Some(Self::SentenceDoctor),
            "grammar-match" => Some(Self::GrammarMatch),
            "story-mode" => Some(Self::StoryMode),
            "conjugation-race" => Some(Self::ConjugationRace),
            "irab-analyzer" => Some(Self::IrabAnalyzer),
            _ => None,
        }
    }
}

/// Mistake record structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMistake {
    pub question_id: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub timestamp: i64,
}
