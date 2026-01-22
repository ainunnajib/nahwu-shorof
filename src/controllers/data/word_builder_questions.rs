use rand::seq::SliceRandom;
use serde::Serialize;
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize)]
pub struct WordBuilderQuestion {
    pub id: String,
    pub root: String,
    pub root_letters: Vec<String>,
    pub pattern: String,
    pub target_word: String,
    pub meaning_ar: String,
    pub meaning_id: String,
    pub meaning_en: String,
    pub hints: Vec<String>,
}

static QUESTIONS: LazyLock<Vec<WordBuilderQuestion>> = LazyLock::new(|| {
    vec![
        WordBuilderQuestion {
            id: "wb-001".to_string(),
            root: "ك-ت-ب".to_string(),
            root_letters: vec!["ك".to_string(), "ت".to_string(), "ب".to_string()],
            pattern: "فَاعِل".to_string(),
            target_word: "كَاتِب".to_string(),
            meaning_ar: "كاتب - الشخص الذي يكتب".to_string(),
            meaning_id: "Penulis - orang yang menulis".to_string(),
            meaning_en: "Writer - one who writes".to_string(),
            hints: vec!["Isim fa'il (kata pelaku)".to_string(), "اسم الفاعل".to_string()],
        },
        WordBuilderQuestion {
            id: "wb-002".to_string(),
            root: "ك-ت-ب".to_string(),
            root_letters: vec!["ك".to_string(), "ت".to_string(), "ب".to_string()],
            pattern: "مَفْعُول".to_string(),
            target_word: "مَكْتُوب".to_string(),
            meaning_ar: "مكتوب - الشيء الذي كُتِب".to_string(),
            meaning_id: "Tulisan / yang ditulis".to_string(),
            meaning_en: "Written - something that is written".to_string(),
            hints: vec!["Isim maf'ul (kata objek)".to_string(), "اسم المفعول".to_string()],
        },
        WordBuilderQuestion {
            id: "wb-003".to_string(),
            root: "ع-ل-م".to_string(),
            root_letters: vec!["ع".to_string(), "ل".to_string(), "م".to_string()],
            pattern: "فَاعِل".to_string(),
            target_word: "عَالِم".to_string(),
            meaning_ar: "عالِم - الشخص الذي يعلم".to_string(),
            meaning_id: "Orang alim / ilmuwan".to_string(),
            meaning_en: "Scholar - one who knows".to_string(),
            hints: vec!["Isim fa'il".to_string(), "اسم الفاعل".to_string()],
        },
        WordBuilderQuestion {
            id: "wb-004".to_string(),
            root: "ع-ل-م".to_string(),
            root_letters: vec!["ع".to_string(), "ل".to_string(), "م".to_string()],
            pattern: "مَفْعُول".to_string(),
            target_word: "مَعْلُوم".to_string(),
            meaning_ar: "معلوم - الشيء المعروف".to_string(),
            meaning_id: "Yang diketahui".to_string(),
            meaning_en: "Known - something that is known".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-005".to_string(),
            root: "د-ر-س".to_string(),
            root_letters: vec!["د".to_string(), "ر".to_string(), "س".to_string()],
            pattern: "فَاعِل".to_string(),
            target_word: "دَارِس".to_string(),
            meaning_ar: "دارس - الشخص الذي يدرس".to_string(),
            meaning_id: "Pelajar - orang yang belajar".to_string(),
            meaning_en: "Student - one who studies".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-006".to_string(),
            root: "ج-ل-س".to_string(),
            root_letters: vec!["ج".to_string(), "ل".to_string(), "س".to_string()],
            pattern: "فَاعِل".to_string(),
            target_word: "جَالِس".to_string(),
            meaning_ar: "جالس - الشخص الذي يجلس".to_string(),
            meaning_id: "Orang yang duduk".to_string(),
            meaning_en: "Sitting - one who sits".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-007".to_string(),
            root: "ق-ر-أ".to_string(),
            root_letters: vec!["ق".to_string(), "ر".to_string(), "أ".to_string()],
            pattern: "فَاعِل".to_string(),
            target_word: "قَارِئ".to_string(),
            meaning_ar: "قارئ - الشخص الذي يقرأ".to_string(),
            meaning_id: "Pembaca - orang yang membaca".to_string(),
            meaning_en: "Reader - one who reads".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-008".to_string(),
            root: "س-م-ع".to_string(),
            root_letters: vec!["س".to_string(), "م".to_string(), "ع".to_string()],
            pattern: "فَاعِل".to_string(),
            target_word: "سَامِع".to_string(),
            meaning_ar: "سامع - الشخص الذي يسمع".to_string(),
            meaning_id: "Pendengar - orang yang mendengar".to_string(),
            meaning_en: "Listener - one who listens".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-009".to_string(),
            root: "ن-ص-ر".to_string(),
            root_letters: vec!["ن".to_string(), "ص".to_string(), "ر".to_string()],
            pattern: "فَاعِل".to_string(),
            target_word: "نَاصِر".to_string(),
            meaning_ar: "ناصر - الشخص الذي ينصر".to_string(),
            meaning_id: "Penolong - orang yang menolong".to_string(),
            meaning_en: "Helper - one who helps".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-010".to_string(),
            root: "ف-ت-ح".to_string(),
            root_letters: vec!["ف".to_string(), "ت".to_string(), "ح".to_string()],
            pattern: "مَفْعُول".to_string(),
            target_word: "مَفْتُوح".to_string(),
            meaning_ar: "مفتوح - الشيء الذي فُتِح".to_string(),
            meaning_id: "Terbuka - yang dibuka".to_string(),
            meaning_en: "Opened - something that is opened".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-011".to_string(),
            root: "ك-ت-ب".to_string(),
            root_letters: vec!["ك".to_string(), "ت".to_string(), "ب".to_string()],
            pattern: "مَفْعَل".to_string(),
            target_word: "مَكْتَب".to_string(),
            meaning_ar: "مكتب - مكان الكتابة".to_string(),
            meaning_id: "Meja tulis / kantor".to_string(),
            meaning_en: "Office/desk - place of writing".to_string(),
            hints: vec!["Isim makan (kata tempat)".to_string(), "اسم المكان".to_string()],
        },
        WordBuilderQuestion {
            id: "wb-012".to_string(),
            root: "ج-ل-س".to_string(),
            root_letters: vec!["ج".to_string(), "ل".to_string(), "س".to_string()],
            pattern: "مَفْعِل".to_string(),
            target_word: "مَجْلِس".to_string(),
            meaning_ar: "مجلس - مكان الجلوس".to_string(),
            meaning_id: "Majelis - tempat duduk".to_string(),
            meaning_en: "Council/assembly - place of sitting".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-013".to_string(),
            root: "س-ج-د".to_string(),
            root_letters: vec!["س".to_string(), "ج".to_string(), "د".to_string()],
            pattern: "مَفْعِل".to_string(),
            target_word: "مَسْجِد".to_string(),
            meaning_ar: "مسجد - مكان السجود".to_string(),
            meaning_id: "Masjid - tempat sujud".to_string(),
            meaning_en: "Mosque - place of prostration".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-014".to_string(),
            root: "ع-ب-د".to_string(),
            root_letters: vec!["ع".to_string(), "ب".to_string(), "د".to_string()],
            pattern: "فَاعِل".to_string(),
            target_word: "عَابِد".to_string(),
            meaning_ar: "عابد - الشخص الذي يعبد".to_string(),
            meaning_id: "Ahli ibadah - orang yang beribadah".to_string(),
            meaning_en: "Worshipper - one who worships".to_string(),
            hints: vec![],
        },
        WordBuilderQuestion {
            id: "wb-015".to_string(),
            root: "ش-ر-ب".to_string(),
            root_letters: vec!["ش".to_string(), "ر".to_string(), "ب".to_string()],
            pattern: "مَفْعُول".to_string(),
            target_word: "مَشْرُوب".to_string(),
            meaning_ar: "مشروب - الشيء الذي يُشرب".to_string(),
            meaning_id: "Minuman - yang diminum".to_string(),
            meaning_en: "Beverage - something to drink".to_string(),
            hints: vec![],
        },
    ]
});

/// Get random questions
pub fn get_random_questions(count: usize) -> Vec<WordBuilderQuestion> {
    let mut rng = rand::thread_rng();
    let mut questions: Vec<_> = QUESTIONS.iter().cloned().collect();
    questions.shuffle(&mut rng);
    questions.truncate(count);
    questions
}

/// Get questions by level
pub fn get_questions_by_level(level: i32) -> Vec<WordBuilderQuestion> {
    if level == 1 {
        QUESTIONS.iter().take(10).cloned().collect()
    } else {
        QUESTIONS.iter().skip(10).cloned().collect()
    }
}
