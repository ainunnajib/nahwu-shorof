use serde::Serialize;
use std::collections::HashMap;
use std::sync::LazyLock;

use super::nahwu_lessons::{Lesson, LessonSection, TopicInfo, WordBreakdown};

type LessonMap = HashMap<String, HashMap<String, Lesson>>;

static SHOROF_LESSONS: LazyLock<LessonMap> = LazyLock::new(|| {
    let mut m: LessonMap = HashMap::new();

    let mut id_lessons = HashMap::new();

    id_lessons.insert("akar-kata".to_string(), Lesson {
        id: "akar-kata".to_string(),
        title: "Akar Kata".to_string(),
        description: "Pelajari konsep dasar huruf asal (akar kata) dalam bahasa Arab".to_string(),
        sections: vec![
            LessonSection {
                section_type: "explanation".to_string(),
                title: Some("Apa itu Akar Kata?".to_string()),
                content: "Dalam bahasa Arab, kebanyakan kata berasal dari tiga huruf dasar yang disebut \"akar kata\" atau \"huruf asli\" (حروف أصلية). Dari akar ini, berbagai kata dengan makna terkait dapat dibentuk dengan menambahkan pola (wazan) tertentu.".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "example".to_string(),
                title: Some("Contoh: Akar ك-ت-ب".to_string()),
                content: "Akar ك-ت-ب (ka-ta-ba) berhubungan dengan makna \"menulis\". Perhatikan kata-kata yang terbentuk:".to_string(),
                arabic_example: None,
                breakdown: Some(vec![
                    WordBreakdown {
                        word: "كَتَبَ".to_string(),
                        role: "Fi'il".to_string(),
                        explanation: "Kata kerja: dia (lk) menulis".to_string(),
                    },
                    WordBreakdown {
                        word: "كَاتِب".to_string(),
                        role: "Isim Fa'il".to_string(),
                        explanation: "Pelaku: penulis".to_string(),
                    },
                    WordBreakdown {
                        word: "مَكْتُوب".to_string(),
                        role: "Isim Maf'ul".to_string(),
                        explanation: "Yang ditulis: tulisan/surat".to_string(),
                    },
                    WordBreakdown {
                        word: "مَكْتَب".to_string(),
                        role: "Isim Makan".to_string(),
                        explanation: "Tempat menulis: meja/kantor".to_string(),
                    },
                    WordBreakdown {
                        word: "كِتَاب".to_string(),
                        role: "Isim".to_string(),
                        explanation: "Buku".to_string(),
                    },
                    WordBreakdown {
                        word: "مَكْتَبَة".to_string(),
                        role: "Isim Makan".to_string(),
                        explanation: "Perpustakaan".to_string(),
                    },
                ]),
            },
            LessonSection {
                section_type: "rule".to_string(),
                title: Some("Kaidah Penting".to_string()),
                content: "1. Kebanyakan akar kata terdiri dari 3 huruf (tsulatsi)\n2. Huruf asal tidak berubah, hanya ditambah huruf tambahan\n3. Memahami akar kata membantu menebak makna kata baru\n4. Huruf tambahan biasanya: ا، و، ي، م، ت، ن، س، ه".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "practice".to_string(),
                title: Some("Latihan".to_string()),
                content: "Mainkan Word Builder untuk berlatih membentuk kata dari akar dan pola!".to_string(),
                arabic_example: None,
                breakdown: None,
            },
        ],
    });

    id_lessons.insert("wazan-fail".to_string(), Lesson {
        id: "wazan-fail".to_string(),
        title: "Wazan Fa'il".to_string(),
        description: "Pelajari pola fa'il untuk membentuk isim fa'il (kata pelaku)".to_string(),
        sections: vec![
            LessonSection {
                section_type: "explanation".to_string(),
                title: Some("Apa itu Wazan Fa'il?".to_string()),
                content: "Wazan فَاعِل (fa'il) adalah pola untuk membentuk isim fa'il, yaitu kata benda yang menunjukkan pelaku perbuatan. Pola ini dibentuk dengan menambahkan alif setelah huruf pertama akar kata.".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "example".to_string(),
                title: Some("Contoh Pembentukan".to_string()),
                content: "Perhatikan bagaimana akar kata menjadi isim fa'il:".to_string(),
                arabic_example: None,
                breakdown: Some(vec![
                    WordBreakdown {
                        word: "كَاتِب".to_string(),
                        role: "ك-ت-ب".to_string(),
                        explanation: "Penulis (dari akar ك-ت-ب = menulis)".to_string(),
                    },
                    WordBreakdown {
                        word: "عَالِم".to_string(),
                        role: "ع-ل-م".to_string(),
                        explanation: "Orang berilmu (dari akar ع-ل-م = mengetahui)".to_string(),
                    },
                    WordBreakdown {
                        word: "قَارِئ".to_string(),
                        role: "ق-ر-أ".to_string(),
                        explanation: "Pembaca (dari akar ق-ر-أ = membaca)".to_string(),
                    },
                    WordBreakdown {
                        word: "سَامِع".to_string(),
                        role: "س-م-ع".to_string(),
                        explanation: "Pendengar (dari akar س-م-ع = mendengar)".to_string(),
                    },
                ]),
            },
            LessonSection {
                section_type: "rule".to_string(),
                title: Some("Rumus Pembentukan".to_string()),
                content: "Rumus: فَاعِل\n- ف = huruf pertama akar (dengan fathah)\n- ا = alif tambahan\n- ع = huruf kedua akar (dengan kasrah)\n- ل = huruf ketiga akar\n\nContoh: ك-ت-ب → كَـ + ا + تِـ + ب → كَاتِب".to_string(),
                arabic_example: None,
                breakdown: None,
            },
        ],
    });

    id_lessons.insert("wazan-maful".to_string(), Lesson {
        id: "wazan-maful".to_string(),
        title: "Wazan Maf'ul".to_string(),
        description: "Pelajari pola maf'ul untuk membentuk isim maf'ul (kata objek)".to_string(),
        sections: vec![
            LessonSection {
                section_type: "explanation".to_string(),
                title: Some("Apa itu Wazan Maf'ul?".to_string()),
                content: "Wazan مَفْعُول (maf'ul) adalah pola untuk membentuk isim maf'ul, yaitu kata benda yang menunjukkan sesuatu yang dikenai perbuatan. Pola ini menambahkan mim di awal dan waw sebelum huruf terakhir.".to_string(),
                arabic_example: None,
                breakdown: None,
            },
            LessonSection {
                section_type: "example".to_string(),
                title: Some("Contoh Pembentukan".to_string()),
                content: "Perhatikan bagaimana akar kata menjadi isim maf'ul:".to_string(),
                arabic_example: None,
                breakdown: Some(vec![
                    WordBreakdown {
                        word: "مَكْتُوب".to_string(),
                        role: "ك-ت-ب".to_string(),
                        explanation: "Yang ditulis (dari akar ك-ت-ب)".to_string(),
                    },
                    WordBreakdown {
                        word: "مَعْلُوم".to_string(),
                        role: "ع-ل-م".to_string(),
                        explanation: "Yang diketahui (dari akar ع-ل-م)".to_string(),
                    },
                    WordBreakdown {
                        word: "مَفْتُوح".to_string(),
                        role: "ف-ت-ح".to_string(),
                        explanation: "Yang dibuka (dari akar ف-ت-ح)".to_string(),
                    },
                    WordBreakdown {
                        word: "مَشْرُوب".to_string(),
                        role: "ش-ر-ب".to_string(),
                        explanation: "Yang diminum / minuman (dari akar ش-ر-ب)".to_string(),
                    },
                ]),
            },
            LessonSection {
                section_type: "rule".to_string(),
                title: Some("Rumus Pembentukan".to_string()),
                content: "Rumus: مَفْعُول\n- مَـ = mim dengan fathah (tambahan)\n- ف = huruf pertama akar (sukun)\n- ع = huruf kedua akar (dengan dhammah)\n- و = waw tambahan\n- ل = huruf ketiga akar\n\nContoh: ك-ت-ب → مَـ + كْـ + تُـ + و + ب → مَكْتُوب".to_string(),
                arabic_example: None,
                breakdown: None,
            },
        ],
    });

    m.insert("id".to_string(), id_lessons);
    m
});

/// Get list of shorof topics
pub fn get_topic_list(lang: &str) -> Vec<TopicInfo> {
    vec![
        TopicInfo {
            id: "akar-kata".to_string(),
            title: match lang {
                "ar" => "جذور الكلمات".to_string(),
                "en" => "Word Roots".to_string(),
                _ => "Akar Kata".to_string(),
            },
            description: match lang {
                "ar" => "تعلم مفهوم الحروف الأصلية".to_string(),
                "en" => "Learn the concept of root letters".to_string(),
                _ => "Memahami huruf asli".to_string(),
            },
            order: 1,
            locked: false,
        },
        TopicInfo {
            id: "wazan-fail".to_string(),
            title: match lang {
                "ar" => "وزن فاعل".to_string(),
                "en" => "Fa'il Pattern".to_string(),
                _ => "Wazan Fa'il".to_string(),
            },
            description: match lang {
                "ar" => "تعلم وزن فاعل لتكوين اسم الفاعل".to_string(),
                "en" => "Learn the fa'il pattern".to_string(),
                _ => "Pola isim fa'il".to_string(),
            },
            order: 2,
            locked: false,
        },
        TopicInfo {
            id: "wazan-maful".to_string(),
            title: match lang {
                "ar" => "وزن مفعول".to_string(),
                "en" => "Maf'ul Pattern".to_string(),
                _ => "Wazan Maf'ul".to_string(),
            },
            description: match lang {
                "ar" => "تعلم وزن مفعول لتكوين اسم المفعول".to_string(),
                "en" => "Learn the maf'ul pattern".to_string(),
                _ => "Pola isim maf'ul".to_string(),
            },
            order: 3,
            locked: false,
        },
        TopicInfo {
            id: "isim-makan".to_string(),
            title: match lang {
                "ar" => "اسم المكان".to_string(),
                "en" => "Noun of Place".to_string(),
                _ => "Isim Makan".to_string(),
            },
            description: match lang {
                "ar" => "تعلم كيفية تكوين اسم المكان".to_string(),
                "en" => "Learn how to form nouns of place".to_string(),
                _ => "Kata benda tempat".to_string(),
            },
            order: 4,
            locked: true,
        },
    ]
}

/// Get a specific lesson
pub fn get_lesson(topic_id: &str) -> Option<Lesson> {
    SHOROF_LESSONS
        .get("id")
        .and_then(|lessons| lessons.get(topic_id))
        .cloned()
}
