# Nahwu Shorof - Arabic Grammar Learning Platform

A Rust web application built with the Loco framework that teaches Arabic grammar (Nahwu and Shorof) through interactive lessons and games. This is a Rust port of the original Next.js application at https://nahwushorof.netlify.app/.

## Project Structure

```
nahwu-shorof/
├── src/
│   ├── app.rs                 # Loco app hooks and configuration
│   ├── lib.rs                 # Library exports
│   ├── bin/main.rs            # CLI entry point
│   ├── controllers/
│   │   ├── mod.rs             # Controller exports
│   │   ├── home.rs            # Home page controller
│   │   ├── learn.rs           # Learning section controller
│   │   ├── play.rs            # Games section controller
│   │   ├── profile.rs         # User profile controller
│   │   ├── api.rs             # JSON API endpoints
│   │   └── data/
│   │       ├── mod.rs
│   │       ├── translations.rs      # i18n translations (ID, AR, EN)
│   │       ├── nahwu_lessons.rs     # Nahwu lesson content
│   │       ├── shorof_lessons.rs    # Shorof lesson content
│   │       ├── word_builder_questions.rs    # Word Builder game data
│   │       └── sentence_doctor_questions.rs # Sentence Doctor game data
│   ├── models/
│   │   ├── mod.rs
│   │   ├── _entities/         # SeaORM entities
│   │   ├── users.rs
│   │   ├── topics.rs
│   │   ├── progress.rs
│   │   ├── game_sessions.rs
│   │   ├── achievements.rs
│   │   └── user_achievements.rs
│   └── views/
│       └── mod.rs             # Tera template engine setup
├── migration/
│   └── src/
│       ├── lib.rs
│       ├── m20240101_000001_create_users.rs
│       ├── m20240101_000002_create_topics.rs
│       ├── m20240101_000003_create_progress.rs
│       ├── m20240101_000004_create_game_sessions.rs
│       └── m20240101_000005_create_achievements.rs
├── assets/
│   ├── views/
│   │   ├── partials/base.html      # Base template with nav
│   │   ├── home/index.html         # Home page
│   │   ├── learn/index.html        # Learning topics list
│   │   ├── learn/topic.html        # Individual lesson page
│   │   ├── play/index.html         # Games list
│   │   ├── play/word_builder.html  # Word Builder game
│   │   ├── play/sentence_doctor.html # Sentence Doctor game
│   │   └── profile/index.html      # User profile
│   └── static/
│       ├── 404.html
│       └── styles.css
├── config/
│   └── development.yaml       # Loco configuration
├── Cargo.toml
└── .gitignore
```

## Features

### Learning Section (Nahwu & Shorof)
- **Nahwu** (النحو): Arabic sentence structure
  - Mubtada-Khabar (المبتدأ والخبر) - Subject and Predicate
  - Fi'il-Fa'il (الفعل والفاعل) - Verb and Subject
  - Maf'ul Bih (المفعول به) - Direct Object

- **Shorof** (الصرف): Arabic word morphology
  - Wazan patterns (فَعَلَ، فَاعِل، مَفْعُول، etc.)
  - Verb conjugations and derivations
  - Root letter system

### Games Section
- **Word Builder** (Pembangun Kata): Build Arabic words from root letters and wazan patterns
- **Sentence Doctor** (Dokter Kalimat): Find and fix i'rab (grammatical case) errors in sentences

### Multi-language Support
- Indonesian (id) - default
- Arabic (ar)
- English (en)

Use `?lang=ar` or `?lang=en` query parameter to switch languages.

## Routes

| Route | Description |
|-------|-------------|
| `GET /` | Home page |
| `GET /learn` | Learning topics list |
| `GET /learn/nahwu/{topic_id}` | Nahwu lesson page |
| `GET /learn/shorof/{topic_id}` | Shorof lesson page |
| `GET /play` | Games list |
| `GET /play/word-builder` | Word Builder game |
| `GET /play/sentence-doctor` | Sentence Doctor game |
| `GET /profile` | User profile |
| `GET /api/games/word-builder/questions` | Word Builder questions API |
| `GET /api/games/sentence-doctor/questions` | Sentence Doctor questions API |
| `POST /api/games/submit-score` | Submit game score |

## Tech Stack

- **Framework**: Loco 0.14 (Rust web framework)
- **Database**: SQLite with SeaORM
- **Templates**: Tera templating engine
- **Styling**: Tailwind CSS (via CDN)
- **Fonts**: Inter (UI) + Amiri (Arabic text)

## Development

### Prerequisites
- Rust (latest stable)
- Cargo

### Running the Application

```bash
# Build
cargo build

# Run development server
cargo run start

# Server runs at http://localhost:5150
```

### Configuration

Edit `config/development.yaml` for:
- Server port (default: 5150)
- Database connection
- CORS settings
- Static file serving

## Database Schema

- **users**: User accounts with XP, level, streak tracking
- **topics**: Nahwu/Shorof topic definitions
- **progress**: User progress per topic
- **game_sessions**: Game play history
- **achievements**: Achievement definitions
- **user_achievements**: User's earned achievements

## Original Source

This is a Rust/Loco port of the original Next.js application:
- Live: https://nahwushorof.netlify.app/
- Source: https://github.com/ainunnajib/afkaaruna

## Build Notes

Key changes made for Loco 0.14 compatibility:
- `Hooks` trait requires `connect_workers`, `register_tasks`, `truncate`, `seed` methods
- `boot()` function takes 3 parameters: `mode`, `environment`, `config`
- Axum 0.8 uses `{param}` syntax instead of `:param` for route parameters
- Static folder requires `404.html` fallback file
