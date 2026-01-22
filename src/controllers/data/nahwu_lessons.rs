use serde::Serialize;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize)]
pub struct TopicInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub order: i32,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonSection {
    pub section_type: String,
    pub title: Option<String>,
    pub content: String,
    pub arabic_example: Option<String>,
    pub breakdown: Option<Vec<WordBreakdown>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WordBreakdown {
    pub word: String,
    pub role: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub description: String,
    pub sections: Vec<LessonSection>,
}

type LessonMap = HashMap<String, HashMap<String, Lesson>>;

static NAHWU_LESSONS: LazyLock<LessonMap> = LazyLock::new(|| {
    let mut m: LessonMap = HashMap::new();

    // Indonesian lessons
    let mut id_lessons = HashMap::new();
    id_lessons.insert("mubtada-khabar".to_string(), Lesson {
        id: "mubtada-khabar".to_string(),
        title: "Mubtada & Khabar".to_string(),
        description: "Pelajari struktur dasar kalimat nominal (jumlah ismiyyah) dalam bahasa Arab".to_string(),
        sections: vec![
            LessonSection {
                section_type: "explanation".to_string(),
                title: Some("Apa itu Mubtada?".to_string()),
                content: "Mubtada adalah kata benda yang menjadi subjek atau pokok pembicaraan dalam kalimat. Mubtada selalu dalam keadaan rafa' (marfu') ditandai dengan dhammah.".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "explanation".to_string(),
                title: Some("Apa itu Khabar?".to_string()),
                content: "Khabar adalah kata atau frasa yang memberikan informasi tentang mubtada. Khabar juga dalam keadaan rafa' (marfu').".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "example".to_string(),
                title: Some("Contoh 1".to_string()),
                content: "Perhatikan contoh kalimat berikut:".to_string(),
                arabic_example: Some("الطَّالِبُ مُجْتَهِدٌ".to_string()),
                breakdown: Some(vec![
                    WordBreakdown {
                        word: "الطَّالِبُ".to_string(),
                        role: "Mubtada".to_string(),
                        explanation: "Mubtada, marfu' dengan dhammah. Artinya: murid/pelajar".to_string(),
                    },
                    WordBreakdown {
                        word: "مُجْتَهِدٌ".to_string(),
                        role: "Khabar".to_string(),
                        explanation: "Khabar, marfu' dengan dhammah. Artinya: rajin".to_string(),
                    },
                ]),
            },
            LessonSection {
                section_type: "example".to_string(),
                title: Some("Contoh 2".to_string()),
                content: "Contoh lain:".to_string(),
                arabic_example: Some("البَيْتُ كَبِيرٌ".to_string()),
                breakdown: Some(vec![
                    WordBreakdown {
                        word: "البَيْتُ".to_string(),
                        role: "Mubtada".to_string(),
                        explanation: "Mubtada, marfu' dengan dhammah. Artinya: rumah".to_string(),
                    },
                    WordBreakdown {
                        word: "كَبِيرٌ".to_string(),
                        role: "Khabar".to_string(),
                        explanation: "Khabar, marfu' dengan dhammah. Artinya: besar".to_string(),
                    },
                ]),
            },
            LessonSection {
                section_type: "rule".to_string(),
                title: Some("Kaidah Penting".to_string()),
                content: "1. Mubtada dan Khabar keduanya harus marfu' (rafa')\n2. Mubtada biasanya berupa isim ma'rifah (kata benda yang sudah diketahui)\n3. Khabar biasanya berupa isim nakirah (kata benda yang belum diketahui)".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "practice".to_string(),
                title: Some("Latihan".to_string()),
                content: "Sekarang coba mainkan game Sentence Doctor untuk berlatih mengidentifikasi kesalahan i'rab pada mubtada dan khabar!".to_string(),
                arabic_example: None,
                breakdown: None,
            },
        ],
    });

    id_lessons.insert("fiil-fail".to_string(), Lesson {
        id: "fiil-fail".to_string(),
        title: "Fi'il & Fa'il".to_string(),
        description: "Pelajari struktur kalimat verbal (jumlah fi'liyyah) dalam bahasa Arab".to_string(),
        sections: vec![
            LessonSection {
                section_type: "explanation".to_string(),
                title: Some("Apa itu Fi'il?".to_string()),
                content: "Fi'il adalah kata kerja dalam bahasa Arab. Ada tiga jenis fi'il: fi'il madhi (lampau), fi'il mudhari' (sekarang/akan datang), dan fi'il amr (perintah).".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "explanation".to_string(),
                title: Some("Apa itu Fa'il?".to_string()),
                content: "Fa'il adalah pelaku perbuatan (subjek dari kata kerja). Fa'il selalu dalam keadaan rafa' (marfu') ditandai dengan dhammah.".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "example".to_string(),
                title: Some("Contoh".to_string()),
                content: "Perhatikan contoh kalimat fi'liyyah:".to_string(),
                arabic_example: Some("ذَهَبَ الطَّالِبُ إِلَى المَدْرَسَةِ".to_string()),
                breakdown: Some(vec![
                    WordBreakdown {
                        word: "ذَهَبَ".to_string(),
                        role: "Fi'il Madhi".to_string(),
                        explanation: "Kata kerja lampau. Artinya: pergi".to_string(),
                    },
                    WordBreakdown {
                        word: "الطَّالِبُ".to_string(),
                        role: "Fa'il".to_string(),
                        explanation: "Fa'il (pelaku), marfu' dengan dhammah. Artinya: murid".to_string(),
                    },
                    WordBreakdown {
                        word: "إِلَى المَدْرَسَةِ".to_string(),
                        role: "Jar-Majrur".to_string(),
                        explanation: "Huruf jar + majrur. Artinya: ke sekolah".to_string(),
                    },
                ]),
            },
            LessonSection {
                section_type: "rule".to_string(),
                title: Some("Kaidah Penting".to_string()),
                content: "1. Jumlah fi'liyyah dimulai dengan fi'il (kata kerja)\n2. Fa'il selalu marfu' (rafa')\n3. Fa'il bisa berupa isim zhahir (nyata) atau dhamir (kata ganti)".to_string(),
                arabic_example: None,
                breakdown: None,
            },
        ],
    });

    id_lessons.insert("maful-bih".to_string(), Lesson {
        id: "maful-bih".to_string(),
        title: "Maf'ul Bih".to_string(),
        description: "Pelajari tentang objek langsung dalam kalimat bahasa Arab".to_string(),
        sections: vec![
            LessonSection {
                section_type: "explanation".to_string(),
                title: Some("Apa itu Maf'ul Bih?".to_string()),
                content: "Maf'ul bih adalah kata benda yang menjadi objek dari kata kerja transitif (fi'il muta'addi). Maf'ul bih selalu dalam keadaan nashab (manshub) ditandai dengan fathah.".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "example".to_string(),
                title: Some("Contoh".to_string()),
                content: "Perhatikan contoh berikut:".to_string(),
                arabic_example: Some("قَرَأَ المُعَلِّمُ الكِتَابَ".to_string()),
                breakdown: Some(vec![
                    WordBreakdown {
                        word: "قَرَأَ".to_string(),
                        role: "Fi'il".to_string(),
                        explanation: "Kata kerja lampau. Artinya: membaca".to_string(),
                    },
                    WordBreakdown {
                        word: "المُعَلِّمُ".to_string(),
                        role: "Fa'il".to_string(),
                        explanation: "Fa'il, marfu' dengan dhammah. Artinya: guru".to_string(),
                    },
                    WordBreakdown {
                        word: "الكِتَابَ".to_string(),
                        role: "Maf'ul Bih".to_string(),
                        explanation: "Maf'ul bih, manshub dengan fathah. Artinya: buku".to_string(),
                    },
                ]),
            },
            LessonSection {
                section_type: "rule".to_string(),
                title: Some("Kaidah Penting".to_string()),
                content: "1. Maf'ul bih selalu manshub (nashab)\n2. Hanya fi'il muta'addi (transitif) yang memiliki maf'ul bih\n3. Urutan biasanya: Fi'il - Fa'il - Maf'ul Bih".to_string(),
                arabic_example: None,
                breakdown: None,
            },
        ],
    });

    m.insert("id".to_string(), id_lessons);
    m
});

/// Get list of nahwu topics
pub fn get_topic_list(lang: &str) -> Vec<TopicInfo> {
    vec![
        TopicInfo {
            id: "mubtada-khabar".to_string(),
            title: match lang {
                "ar" => "المبتدأ والخبر".to_string(),
                "en" => "Subject & Predicate".to_string(),
                _ => "Mubtada & Khabar".to_string(),
            },
            description: match lang {
                "ar" => "تعلم البنية الأساسية للجملة الاسمية".to_string(),
                "en" => "Learn the basic structure of nominal sentences".to_string(),
                _ => "Jumlah ismiyyah dasar".to_string(),
            },
            order: 1,
            locked: false,
        },
        TopicInfo {
            id: "fiil-fail".to_string(),
            title: match lang {
                "ar" => "الفعل والفاعل".to_string(),
                "en" => "Verb & Subject".to_string(),
                _ => "Fi'il & Fa'il".to_string(),
            },
            description: match lang {
                "ar" => "تعلم بنية الجملة الفعلية".to_string(),
                "en" => "Learn verbal sentence structure".to_string(),
                _ => "Jumlah fi'liyyah dasar".to_string(),
            },
            order: 2,
            locked: false,
        },
        TopicInfo {
            id: "maful-bih".to_string(),
            title: match lang {
                "ar" => "المفعول به".to_string(),
                "en" => "Direct Object".to_string(),
                _ => "Maf'ul Bih".to_string(),
            },
            description: match lang {
                "ar" => "تعلم عن المفعول به".to_string(),
                "en" => "Learn about direct objects".to_string(),
                _ => "Objek langsung dalam kalimat".to_string(),
            },
            order: 3,
            locked: false,
        },
        TopicInfo {
            id: "huruf-jar".to_string(),
            title: match lang {
                "ar" => "حروف الجر".to_string(),
                "en" => "Prepositions".to_string(),
                _ => "Huruf Jar".to_string(),
            },
            description: match lang {
                "ar" => "تعلم حروف الجر وأثرها".to_string(),
                "en" => "Learn prepositions and their effects".to_string(),
                _ => "Kata depan dan pengaruhnya".to_string(),
            },
            order: 4,
            locked: true,
        },
        TopicInfo {
            id: "inna-saudaranya".to_string(),
            title: match lang {
                "ar" => "إنَّ وأخواتها".to_string(),
                "en" => "Inna & Sisters".to_string(),
                _ => "Inna & Saudaranya".to_string(),
            },
            description: match lang {
                "ar" => "تعلم إنَّ وأخواتها".to_string(),
                "en" => "Learn inna and its sisters".to_string(),
                _ => "Huruf yang menashobkan mubtada".to_string(),
            },
            order: 5,
            locked: true,
        },
    ]
}

/// Get a specific lesson
pub fn get_lesson(topic_id: &str) -> Option<Lesson> {
    NAHWU_LESSONS
        .get("id")
        .and_then(|lessons| lessons.get(topic_id))
        .cloned()
}
