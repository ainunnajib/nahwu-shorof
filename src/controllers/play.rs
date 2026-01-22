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

/// Play page - list all games
pub async fn index(Query(query): Query<LangQuery>) -> Result<Html<String>> {
    let lang = query.lang.unwrap_or_else(|| "id".to_string());

    let mut ctx = Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("page", "play");

    // Translations
    ctx.insert("app_name", t("app.name", &lang));
    ctx.insert("nav_home", t("nav.home", &lang));
    ctx.insert("nav_play", t("nav.play", &lang));
    ctx.insert("nav_learn", t("nav.learn", &lang));
    ctx.insert("nav_profile", t("nav.profile", &lang));
    ctx.insert("game_start", t("game.start", &lang));

    // Page content
    ctx.insert("page_title", t("nav.play", &lang));
    ctx.insert("page_subtitle", match lang.as_str() {
        "ar" => "اختر لعبة للبدء في التعلم",
        "en" => "Choose a game to start learning",
        _ => "Pilih permainan untuk mulai belajar"
    });

    // Games list
    let games = vec![
        serde_json::json!({
            "id": "word-builder",
            "name": t("game.wordBuilder", &lang),
            "description": t("game.wordBuilder.desc", &lang),
            "category": "shorof",
            "icon": "puzzle",
            "available": true
        }),
        serde_json::json!({
            "id": "sentence-doctor",
            "name": t("game.sentenceDoctor", &lang),
            "description": t("game.sentenceDoctor.desc", &lang),
            "category": "nahwu",
            "icon": "stethoscope",
            "available": true
        }),
        serde_json::json!({
            "id": "grammar-match",
            "name": t("game.grammarMatch", &lang),
            "description": match lang.as_str() {
                "ar" => "طابق الكلمات مع وظائفها النحوية",
                "en" => "Match words with their grammatical roles",
                _ => "Cocokkan kata dengan peran tata bahasanya"
            },
            "category": "nahwu",
            "icon": "link",
            "available": false
        }),
        serde_json::json!({
            "id": "conjugation-race",
            "name": t("game.conjugationRace", &lang),
            "description": match lang.as_str() {
                "ar" => "تصريف الأفعال بسرعة",
                "en" => "Conjugate verbs quickly",
                _ => "Tashrif kata kerja dengan cepat"
            },
            "category": "shorof",
            "icon": "timer",
            "available": false
        }),
    ];
    ctx.insert("games", &games);

    let html = TEMPLATES.render("play/index.html", &ctx)
        .map_err(|e| Error::string(&format!("Template error: {}", e)))?;

    Ok(Html(html))
}

/// Word Builder game page
pub async fn word_builder(Query(query): Query<LangQuery>) -> Result<Html<String>> {
    let lang = query.lang.unwrap_or_else(|| "id".to_string());

    let mut ctx = Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("page", "play");
    ctx.insert("game", "word-builder");

    // Translations
    ctx.insert("app_name", t("app.name", &lang));
    ctx.insert("nav_home", t("nav.home", &lang));
    ctx.insert("nav_play", t("nav.play", &lang));
    ctx.insert("nav_learn", t("nav.learn", &lang));
    ctx.insert("nav_profile", t("nav.profile", &lang));
    ctx.insert("game_start", t("game.start", &lang));
    ctx.insert("button_check", t("button.check", &lang));
    ctx.insert("button_back", t("button.back", &lang));

    ctx.insert("game_name", t("game.wordBuilder", &lang));
    ctx.insert("game_description", t("game.wordBuilder.desc", &lang));
    ctx.insert("instruction", t("instruction.wordBuilder", &lang));

    // How to play
    let how_to_play = match lang.as_str() {
        "ar" => vec![
            "انظر إلى الجذر والوزن المعطى",
            "اضغط على الحروف لترتيبها",
            "تحقق من إجابتك واستمر"
        ],
        "en" => vec![
            "Look at the given root and pattern",
            "Tap the letters to arrange them",
            "Check your answer and continue"
        ],
        _ => vec![
            "Lihat akar huruf dan pola yang diberikan",
            "Ketuk huruf untuk menyusunnya",
            "Periksa jawaban dan lanjutkan"
        ]
    };
    ctx.insert("how_to_play", &how_to_play);

    let html = TEMPLATES.render("play/word_builder.html", &ctx)
        .map_err(|e| Error::string(&format!("Template error: {}", e)))?;

    Ok(Html(html))
}

/// Sentence Doctor game page
pub async fn sentence_doctor(Query(query): Query<LangQuery>) -> Result<Html<String>> {
    let lang = query.lang.unwrap_or_else(|| "id".to_string());

    let mut ctx = Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("page", "play");
    ctx.insert("game", "sentence-doctor");

    // Translations
    ctx.insert("app_name", t("app.name", &lang));
    ctx.insert("nav_home", t("nav.home", &lang));
    ctx.insert("nav_play", t("nav.play", &lang));
    ctx.insert("nav_learn", t("nav.learn", &lang));
    ctx.insert("nav_profile", t("nav.profile", &lang));
    ctx.insert("game_start", t("game.start", &lang));
    ctx.insert("button_check", t("button.check", &lang));
    ctx.insert("button_back", t("button.back", &lang));

    ctx.insert("game_name", t("game.sentenceDoctor", &lang));
    ctx.insert("game_description", t("game.sentenceDoctor.desc", &lang));
    ctx.insert("instruction", t("instruction.sentenceDoctor", &lang));

    // How to play
    let how_to_play = match lang.as_str() {
        "ar" => vec![
            "انظر إلى الكلمة المميزة باللون الأحمر",
            "اختر الشكل الصحيح للكلمة",
            "اقرأ التفسير لفهم القاعدة"
        ],
        "en" => vec![
            "Look at the word highlighted in red",
            "Choose the correct form of the word",
            "Read the explanation to understand the rule"
        ],
        _ => vec![
            "Perhatikan kata yang ditandai merah",
            "Pilih bentuk kata yang benar dari pilihan",
            "Pelajari penjelasan untuk memahami aturannya"
        ]
    };
    ctx.insert("how_to_play", &how_to_play);

    // I'rab topics for filtering
    let irab_topics = match lang.as_str() {
        "ar" => vec![
            ("mubtada-khabar", "المبتدأ والخبر"),
            ("fiil-fail-maful", "الفعل والفاعل والمفعول به"),
            ("inna-kana", "إنّ وكان"),
            ("idhafa-nat", "الإضافة والنعت"),
        ],
        "en" => vec![
            ("mubtada-khabar", "Mubtada-Khabar"),
            ("fiil-fail-maful", "Fa'il and Maf'ul Bih"),
            ("inna-kana", "Inna and Kana"),
            ("idhafa-nat", "Idhafa and Na't"),
        ],
        _ => vec![
            ("mubtada-khabar", "Mubtada-Khabar"),
            ("fiil-fail-maful", "Fa'il dan Maf'ul Bih"),
            ("inna-kana", "Inna dan Kana"),
            ("idhafa-nat", "Idhafah dan Na't"),
        ]
    };
    ctx.insert("irab_topics", &irab_topics);

    let html = TEMPLATES.render("play/sentence_doctor.html", &ctx)
        .map_err(|e| Error::string(&format!("Template error: {}", e)))?;

    Ok(Html(html))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/play")
        .add("/", get(index))
        .add("/word-builder", get(word_builder))
        .add("/sentence-doctor", get(sentence_doctor))
}
