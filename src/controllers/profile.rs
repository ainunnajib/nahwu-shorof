use axum::{extract::Query, response::Html};
use loco_rs::prelude::*;
use serde::Deserialize;
use tera::Context;

use crate::views::TEMPLATES;
use super::data::translations::t;

#[derive(Debug, Deserialize)]
pub struct LangQuery {
    lang: Option<String>,
}

/// Profile page
pub async fn index(Query(query): Query<LangQuery>) -> Result<Html<String>> {
    let lang = query.lang.unwrap_or_else(|| "id".to_string());

    let mut ctx = Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("page", "profile");

    // Translations
    ctx.insert("app_name", t("app.name", &lang));
    ctx.insert("nav_home", t("nav.home", &lang));
    ctx.insert("nav_play", t("nav.play", &lang));
    ctx.insert("nav_learn", t("nav.learn", &lang));
    ctx.insert("nav_profile", t("nav.profile", &lang));

    // Profile labels
    ctx.insert("profile_level", t("profile.level", &lang));
    ctx.insert("profile_total_xp", t("profile.totalXP", &lang));
    ctx.insert("profile_achievements", t("profile.achievements", &lang));
    ctx.insert("profile_streak", t("profile.streak", &lang));

    // Demo user data (in real app, fetch from database)
    let user = serde_json::json!({
        "name": match lang.as_str() {
            "ar" => "طالب",
            "en" => "Student",
            _ => "Pelajar"
        },
        "level": 3,
        "total_xp": 2450,
        "xp_for_next_level": 1000,
        "current_level_xp": 450,
        "streak": 5,
        "lessons_completed": 12,
        "games_played": 45,
        "accuracy": 86
    });
    ctx.insert("user", &user);

    // Stats labels
    ctx.insert("stat_streak", match lang.as_str() {
        "ar" => "أيام متتالية",
        "en" => "Day Streak",
        _ => "Hari Berturut"
    });
    ctx.insert("stat_lessons", match lang.as_str() {
        "ar" => "دروس",
        "en" => "Lessons",
        _ => "Pelajaran"
    });
    ctx.insert("stat_games", match lang.as_str() {
        "ar" => "ألعاب",
        "en" => "Games",
        _ => "Permainan"
    });
    ctx.insert("stat_accuracy", match lang.as_str() {
        "ar" => "دقة",
        "en" => "Accuracy",
        _ => "Akurasi"
    });

    let html = TEMPLATES.render("profile/index.html", &ctx)
        .map_err(|e| Error::string(&format!("Template error: {}", e)))?;

    Ok(Html(html))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/profile")
        .add("/", get(index))
}
