use axum::{extract::{Path, Query}, response::Html};
use loco_rs::prelude::*;
use serde::Deserialize;
use tera::Context;

use crate::views::TEMPLATES;
use super::data::{translations::t, nahwu_lessons, shorof_lessons};

#[derive(Debug, Deserialize)]
pub struct LangQuery {
    lang: Option<String>,
}

/// Learn page - list all topics
pub async fn index(Query(query): Query<LangQuery>) -> Result<Html<String>> {
    let lang = query.lang.unwrap_or_else(|| "id".to_string());

    let mut ctx = Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("page", "learn");

    // Translations
    ctx.insert("app_name", t("app.name", &lang));
    ctx.insert("nav_home", t("nav.home", &lang));
    ctx.insert("nav_play", t("nav.play", &lang));
    ctx.insert("nav_learn", t("nav.learn", &lang));
    ctx.insert("nav_profile", t("nav.profile", &lang));
    ctx.insert("topic_nahwu", t("topic.nahwu", &lang));
    ctx.insert("topic_nahwu_desc", t("topic.nahwu.desc", &lang));
    ctx.insert("topic_shorof", t("topic.shorof", &lang));
    ctx.insert("topic_shorof_desc", t("topic.shorof.desc", &lang));

    // Page title
    ctx.insert("page_title", t("nav.learn", &lang));
    ctx.insert("page_subtitle", match lang.as_str() {
        "ar" => "تعلم مواد النحو والصرف خطوة بخطوة",
        "en" => "Learn Nahwu and Shorof materials step by step",
        _ => "Pelajari materi Nahwu dan Shorof secara bertahap"
    });

    // Get lessons
    let nahwu_topics = nahwu_lessons::get_topic_list(&lang);
    let shorof_topics = shorof_lessons::get_topic_list(&lang);

    ctx.insert("nahwu_topics", &nahwu_topics);
    ctx.insert("shorof_topics", &shorof_topics);

    let html = TEMPLATES.render("learn/index.html", &ctx)
        .map_err(|e| Error::string(&format!("Template error: {}", e)))?;

    Ok(Html(html))
}

/// Nahwu topic lesson page
pub async fn nahwu_topic(
    Path(topic_id): Path<String>,
    Query(query): Query<LangQuery>,
) -> Result<Html<String>> {
    let lang = query.lang.unwrap_or_else(|| "id".to_string());

    let lesson = nahwu_lessons::get_lesson(&topic_id)
        .ok_or_else(|| Error::string("Lesson not found"))?;

    let mut ctx = Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("page", "learn");
    ctx.insert("category", "nahwu");

    // Translations
    ctx.insert("app_name", t("app.name", &lang));
    ctx.insert("nav_home", t("nav.home", &lang));
    ctx.insert("nav_play", t("nav.play", &lang));
    ctx.insert("nav_learn", t("nav.learn", &lang));
    ctx.insert("nav_profile", t("nav.profile", &lang));
    ctx.insert("button_back", t("button.back", &lang));
    ctx.insert("game_sentence_doctor", t("game.sentenceDoctor", &lang));

    ctx.insert("lesson", &lesson);
    ctx.insert("topic_id", &topic_id);

    let html = TEMPLATES.render("learn/topic.html", &ctx)
        .map_err(|e| Error::string(&format!("Template error: {}", e)))?;

    Ok(Html(html))
}

/// Shorof topic lesson page
pub async fn shorof_topic(
    Path(topic_id): Path<String>,
    Query(query): Query<LangQuery>,
) -> Result<Html<String>> {
    let lang = query.lang.unwrap_or_else(|| "id".to_string());

    let lesson = shorof_lessons::get_lesson(&topic_id)
        .ok_or_else(|| Error::string("Lesson not found"))?;

    let mut ctx = Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("page", "learn");
    ctx.insert("category", "shorof");

    // Translations
    ctx.insert("app_name", t("app.name", &lang));
    ctx.insert("nav_home", t("nav.home", &lang));
    ctx.insert("nav_play", t("nav.play", &lang));
    ctx.insert("nav_learn", t("nav.learn", &lang));
    ctx.insert("nav_profile", t("nav.profile", &lang));
    ctx.insert("button_back", t("button.back", &lang));
    ctx.insert("game_word_builder", t("game.wordBuilder", &lang));

    ctx.insert("lesson", &lesson);
    ctx.insert("topic_id", &topic_id);

    let html = TEMPLATES.render("learn/topic.html", &ctx)
        .map_err(|e| Error::string(&format!("Template error: {}", e)))?;

    Ok(Html(html))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/learn")
        .add("/", get(index))
        .add("/nahwu/:topic_id", get(nahwu_topic))
        .add("/shorof/:topic_id", get(shorof_topic))
}
