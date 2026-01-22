use axum::{extract::Query, response::Html};
use loco_rs::prelude::*;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::views::TEMPLATES;
use super::data::translations::t;

#[derive(Debug, Deserialize)]
pub struct LangQuery {
    lang: Option<String>,
}

/// Home page - landing page
pub async fn index(Query(query): Query<LangQuery>) -> Result<Html<String>> {
    let lang = query.lang.unwrap_or_else(|| "id".to_string());

    let mut ctx = Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("page", "home");

    // Translations
    ctx.insert("app_name", t("app.name", &lang));
    ctx.insert("app_tagline", t("app.tagline", &lang));
    ctx.insert("nav_home", t("nav.home", &lang));
    ctx.insert("nav_play", t("nav.play", &lang));
    ctx.insert("nav_learn", t("nav.learn", &lang));
    ctx.insert("nav_profile", t("nav.profile", &lang));
    ctx.insert("game_start", t("game.start", &lang));
    ctx.insert("nav_learn_btn", t("nav.learn", &lang));

    // Example sentences for home page
    let examples = vec![
        serde_json::json!({
            "arabic": "ذَهَبَ الطَّالِبُ إِلَى المَدْرَسَةِ",
            "translation": match lang.as_str() {
                "ar" => "ذهب الطالب إلى المدرسة",
                "en" => "The student went to school",
                _ => "Murid itu pergi ke sekolah"
            },
            "type": match lang.as_str() {
                "ar" => "جملة فعلية",
                "en" => "Verbal sentence",
                _ => "Contoh kalimat fi'liyyah"
            }
        }),
        serde_json::json!({
            "arabic": "الطَّالِبُ مُجْتَهِدٌ",
            "translation": match lang.as_str() {
                "ar" => "الطالب مجتهد",
                "en" => "The student is diligent",
                _ => "Murid itu rajin"
            },
            "type": match lang.as_str() {
                "ar" => "جملة اسمية",
                "en" => "Nominal sentence",
                _ => "Contoh kalimat ismiyyah"
            }
        }),
    ];
    ctx.insert("examples", &examples);

    let html = TEMPLATES.render("home/index.html", &ctx)
        .map_err(|e| Error::string(&format!("Template error: {}", e)))?;

    Ok(Html(html))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/")
        .add("/", get(index))
}
