# Nahwu Shorof - Arabic Grammar Learning Platform

A Rust web application built with the Loco framework that teaches Arabic grammar (Nahwu and Shorof) through interactive lessons and games.

## Work Summary

### What Was Built
Ported the Next.js Arabic grammar learning app (https://nahwushorof.netlify.app/) to Rust using the Loco web framework. The original source was studied from https://github.com/ainunnajib/afkaaruna.

### Files Created

**Core Application:**
- `src/app.rs` - Loco app hooks (boot, routes, workers, tasks, seed, truncate)
- `src/lib.rs` - Library exports
- `src/bin/main.rs` - CLI entry point
- `Cargo.toml` - Dependencies (loco-rs 0.14, sea-orm, tera, tokio, serde, rand)

**Controllers:**
- `src/controllers/home.rs` - Home page with welcome message and quick actions
- `src/controllers/learn.rs` - Learning section with Nahwu/Shorof topic lists and lesson pages
- `src/controllers/play.rs` - Games section with Word Builder and Sentence Doctor
- `src/controllers/profile.rs` - User profile with stats (XP, level, streak, accuracy)
- `src/controllers/api.rs` - JSON API for game questions and score submission

**Data Modules:**
- `src/controllers/data/translations.rs` - i18n translations (Indonesian, Arabic, English)
- `src/controllers/data/nahwu_lessons.rs` - Nahwu lesson content (Mubtada-Khabar, Fi'il-Fa'il, Maf'ul Bih)
- `src/controllers/data/shorof_lessons.rs` - Shorof lesson content (Wazan patterns)
- `src/controllers/data/word_builder_questions.rs` - 20 Word Builder game questions
- `src/controllers/data/sentence_doctor_questions.rs` - 20 Sentence Doctor game questions

**Database Models & Migrations:**
- `src/models/users.rs` - User model (name, xp, level, streak)
- `src/models/topics.rs` - Topic model (nahwu/shorof categories)
- `src/models/progress.rs` - User progress per topic
- `src/models/game_sessions.rs` - Game play history
- `src/models/achievements.rs` - Achievement definitions
- `src/models/user_achievements.rs` - User's earned achievements
- 5 migration files in `migration/src/`

**Templates (Tera):**
- `assets/views/partials/base.html` - Base layout with bottom navigation
- `assets/views/home/index.html` - Home page
- `assets/views/learn/index.html` - Topic list page
- `assets/views/learn/topic.html` - Individual lesson page
- `assets/views/play/index.html` - Games list
- `assets/views/play/word_builder.html` - Word Builder game UI
- `assets/views/play/sentence_doctor.html` - Sentence Doctor game UI
- `assets/views/profile/index.html` - Profile page

**Static Assets & Config:**
- `assets/static/404.html` - 404 error page
- `assets/static/styles.css` - CSS styles
- `config/development.yaml` - Loco config (port 5150, SQLite, CORS)
- `.gitignore` - Git ignore rules

### Issues Fixed

1. **Loco 0.14 Hooks trait** - Added required methods: `connect_workers`, `register_tasks`, `truncate`, `seed`
2. **boot() signature** - Updated to take 3 params: `mode`, `environment`, `config`
3. **Axum 0.8 routes** - Changed `:param` to `{param}` syntax
4. **Lifetime error** - Fixed `translations.rs` to return static fallback string
5. **Static assets** - Created `assets/static/` folder with `404.html` fallback

### Routes

| Method | Route | Description |
|--------|-------|-------------|
| GET | `/` | Home page |
| GET | `/learn` | Learning topics |
| GET | `/learn/nahwu/{topic_id}` | Nahwu lesson |
| GET | `/learn/shorof/{topic_id}` | Shorof lesson |
| GET | `/play` | Games list |
| GET | `/play/word-builder` | Word Builder game |
| GET | `/play/sentence-doctor` | Sentence Doctor game |
| GET | `/profile` | User profile |
| GET | `/api/games/word-builder/questions` | Word Builder API |
| GET | `/api/games/sentence-doctor/questions` | Sentence Doctor API |
| POST | `/api/games/submit-score` | Submit score API |

### Running

```bash
cargo build
cargo run start
# Server at http://localhost:5150
```

### Multi-language

Add `?lang=ar` (Arabic) or `?lang=en` (English) to any URL. Default is Indonesian.

## Tech Stack

- **Loco 0.14** - Rust web framework
- **SeaORM** - Database ORM
- **SQLite** - Database
- **Tera** - Template engine
- **Tailwind CSS** - Styling (via CDN)
- **Amiri font** - Arabic text

## Repository

https://github.com/ainunnajib/nahwu-shorof
