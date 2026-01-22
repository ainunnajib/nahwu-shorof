use axum::{extract::Query, Json};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use super::data::{word_builder_questions, sentence_doctor_questions};

#[derive(Debug, Deserialize)]
pub struct GameQuery {
    count: Option<usize>,
    level: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: T,
}

/// Get word builder questions
pub async fn word_builder_questions(
    Query(query): Query<GameQuery>,
) -> Result<Json<ApiResponse<Vec<word_builder_questions::WordBuilderQuestion>>>> {
    let count = query.count.unwrap_or(10);
    let questions = word_builder_questions::get_random_questions(count);

    Ok(Json(ApiResponse {
        success: true,
        data: questions,
    }))
}

/// Get sentence doctor questions
pub async fn sentence_doctor_questions(
    Query(query): Query<GameQuery>,
) -> Result<Json<ApiResponse<Vec<sentence_doctor_questions::SentenceDoctorQuestion>>>> {
    let count = query.count.unwrap_or(10);
    let questions = sentence_doctor_questions::get_random_questions(count);

    Ok(Json(ApiResponse {
        success: true,
        data: questions,
    }))
}

#[derive(Debug, Deserialize)]
pub struct SubmitScoreRequest {
    game_type: String,
    score: i32,
    max_score: i32,
    duration: i32,
    mistakes: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct SubmitScoreResponse {
    xp_earned: i32,
    stars: i32,
    level_up: bool,
    new_level: Option<i32>,
}

/// Submit game score
pub async fn submit_score(
    Json(payload): Json<SubmitScoreRequest>,
) -> Result<Json<ApiResponse<SubmitScoreResponse>>> {
    // Calculate XP and stars
    let percentage = if payload.max_score > 0 {
        (payload.score as f32 / payload.max_score as f32) * 100.0
    } else {
        0.0
    };

    let stars = if percentage >= 90.0 {
        3
    } else if percentage >= 70.0 {
        2
    } else if percentage >= 50.0 {
        1
    } else {
        0
    };

    // Base XP + bonus for accuracy
    let base_xp = payload.score * 10;
    let bonus_xp = stars * 50;
    let xp_earned = base_xp + bonus_xp;

    // In a real app, save to database and check for level up
    let response = SubmitScoreResponse {
        xp_earned,
        stars,
        level_up: false,
        new_level: None,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: response,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/games/word-builder/questions", get(word_builder_questions))
        .add("/games/sentence-doctor/questions", get(sentence_doctor_questions))
        .add("/games/submit-score", post(submit_score))
}
