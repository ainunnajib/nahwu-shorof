use rand::seq::SliceRandom;
use serde::Serialize;
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize)]
pub struct SentenceDoctorQuestion {
    pub id: String,
    pub incorrect_sentence: String,
    pub correct_sentence: String,
    pub error_type: String,
    pub error_position: usize,
    pub explanation_ar: String,
    pub explanation_id: String,
    pub explanation_en: String,
    pub options: Vec<String>,
}

static QUESTIONS: LazyLock<Vec<SentenceDoctorQuestion>> = LazyLock::new(|| {
    vec![
        // Mubtada-Khabar errors
        SentenceDoctorQuestion {
            id: "sd-001".to_string(),
            incorrect_sentence: "الطالبَ مجتهدٌ".to_string(),
            correct_sentence: "الطالبُ مجتهدٌ".to_string(),
            error_type: "irab".to_string(),
            error_position: 0,
            explanation_ar: "المبتدأ مرفوع دائماً، والصحيح: الطالبُ (بالضمة)".to_string(),
            explanation_id: "Mubtada selalu marfu' (rafa'). Yang benar: الطالبُ dengan dhammah".to_string(),
            explanation_en: "The subject (mubtada) is always nominative. Correct: الطالبُ with damma".to_string(),
            options: vec!["الطالبُ".to_string(), "الطالبَ".to_string(), "الطالبِ".to_string()],
        },
        SentenceDoctorQuestion {
            id: "sd-002".to_string(),
            incorrect_sentence: "البيتُ كبيرٍ".to_string(),
            correct_sentence: "البيتُ كبيرٌ".to_string(),
            error_type: "irab".to_string(),
            error_position: 1,
            explanation_ar: "الخبر مرفوع مثل المبتدأ، والصحيح: كبيرٌ (بالضمة)".to_string(),
            explanation_id: "Khabar marfu' seperti mubtada. Yang benar: كبيرٌ dengan dhammah".to_string(),
            explanation_en: "The predicate is nominative like the subject. Correct: كبيرٌ with damma".to_string(),
            options: vec!["كبيرٌ".to_string(), "كبيرٍ".to_string(), "كبيرَ".to_string()],
        },
        SentenceDoctorQuestion {
            id: "sd-003".to_string(),
            incorrect_sentence: "المعلمُ نشيطَ".to_string(),
            correct_sentence: "المعلمُ نشيطٌ".to_string(),
            error_type: "irab".to_string(),
            error_position: 1,
            explanation_ar: "الخبر يجب أن يكون مرفوعاً: نشيطٌ".to_string(),
            explanation_id: "Khabar harus marfu': نشيطٌ".to_string(),
            explanation_en: "The predicate must be nominative: نشيطٌ".to_string(),
            options: vec!["نشيطٌ".to_string(), "نشيطَ".to_string(), "نشيطِ".to_string()],
        },
        // Fa'il errors
        SentenceDoctorQuestion {
            id: "sd-004".to_string(),
            incorrect_sentence: "ذهبَ الولدَ إلى المدرسةِ".to_string(),
            correct_sentence: "ذهبَ الولدُ إلى المدرسةِ".to_string(),
            error_type: "irab".to_string(),
            error_position: 1,
            explanation_ar: "الفاعل مرفوع دائماً، والصحيح: الولدُ (بالضمة)".to_string(),
            explanation_id: "Fa'il (pelaku) selalu marfu'. Yang benar: الولدُ dengan dhammah".to_string(),
            explanation_en: "The subject of the verb is always nominative. Correct: الولدُ with damma".to_string(),
            options: vec!["الولدُ".to_string(), "الولدَ".to_string(), "الولدِ".to_string()],
        },
        SentenceDoctorQuestion {
            id: "sd-005".to_string(),
            incorrect_sentence: "كتبَ الطالبِ الدرسَ".to_string(),
            correct_sentence: "كتبَ الطالبُ الدرسَ".to_string(),
            error_type: "irab".to_string(),
            error_position: 1,
            explanation_ar: "الفاعل مرفوع: الطالبُ".to_string(),
            explanation_id: "Fa'il harus marfu': الطالبُ".to_string(),
            explanation_en: "The subject must be nominative: الطالبُ".to_string(),
            options: vec!["الطالبُ".to_string(), "الطالبِ".to_string(), "الطالبَ".to_string()],
        },
        // Maf'ul bih errors
        SentenceDoctorQuestion {
            id: "sd-006".to_string(),
            incorrect_sentence: "قرأَ المعلمُ الكتابُ".to_string(),
            correct_sentence: "قرأَ المعلمُ الكتابَ".to_string(),
            error_type: "irab".to_string(),
            error_position: 2,
            explanation_ar: "المفعول به منصوب دائماً، والصحيح: الكتابَ (بالفتحة)".to_string(),
            explanation_id: "Maf'ul bih (objek) selalu manshub. Yang benar: الكتابَ dengan fathah".to_string(),
            explanation_en: "The object is always accusative. Correct: الكتابَ with fatha".to_string(),
            options: vec!["الكتابَ".to_string(), "الكتابُ".to_string(), "الكتابِ".to_string()],
        },
        SentenceDoctorQuestion {
            id: "sd-007".to_string(),
            incorrect_sentence: "شربَ الولدُ الماءُ".to_string(),
            correct_sentence: "شربَ الولدُ الماءَ".to_string(),
            error_type: "irab".to_string(),
            error_position: 2,
            explanation_ar: "المفعول به منصوب: الماءَ".to_string(),
            explanation_id: "Maf'ul bih harus manshub: الماءَ".to_string(),
            explanation_en: "The object must be accusative: الماءَ".to_string(),
            options: vec!["الماءَ".to_string(), "الماءُ".to_string(), "الماءِ".to_string()],
        },
        // Jar-Majrur errors
        SentenceDoctorQuestion {
            id: "sd-008".to_string(),
            incorrect_sentence: "سافرَ إلى المدينةُ".to_string(),
            correct_sentence: "سافرَ إلى المدينةِ".to_string(),
            error_type: "irab".to_string(),
            error_position: 2,
            explanation_ar: "الاسم بعد حرف الجر مجرور، والصحيح: المدينةِ (بالكسرة)".to_string(),
            explanation_id: "Kata setelah huruf jar harus majrur. Yang benar: المدينةِ dengan kasrah".to_string(),
            explanation_en: "Nouns after prepositions are genitive. Correct: المدينةِ with kasra".to_string(),
            options: vec!["المدينةِ".to_string(), "المدينةُ".to_string(), "المدينةَ".to_string()],
        },
        SentenceDoctorQuestion {
            id: "sd-009".to_string(),
            incorrect_sentence: "خرجَ منَ البيتُ".to_string(),
            correct_sentence: "خرجَ منَ البيتِ".to_string(),
            error_type: "irab".to_string(),
            error_position: 2,
            explanation_ar: "الاسم بعد حرف الجر مجرور: البيتِ".to_string(),
            explanation_id: "Kata setelah huruf jar harus majrur: البيتِ".to_string(),
            explanation_en: "Noun after preposition must be genitive: البيتِ".to_string(),
            options: vec!["البيتِ".to_string(), "البيتُ".to_string(), "البيتَ".to_string()],
        },
        // Inna errors
        SentenceDoctorQuestion {
            id: "sd-010".to_string(),
            incorrect_sentence: "إنَّ الطالبُ مجتهدٌ".to_string(),
            correct_sentence: "إنَّ الطالبَ مجتهدٌ".to_string(),
            error_type: "irab".to_string(),
            error_position: 1,
            explanation_ar: "اسم إنَّ منصوب، والصحيح: الطالبَ (بالفتحة)".to_string(),
            explanation_id: "Isim inna selalu manshub. Yang benar: الطالبَ dengan fathah".to_string(),
            explanation_en: "The noun after inna is accusative. Correct: الطالبَ with fatha".to_string(),
            options: vec!["الطالبَ".to_string(), "الطالبُ".to_string(), "الطالبِ".to_string()],
        },
        SentenceDoctorQuestion {
            id: "sd-011".to_string(),
            incorrect_sentence: "إنَّ العلمُ نورٌ".to_string(),
            correct_sentence: "إنَّ العلمَ نورٌ".to_string(),
            error_type: "irab".to_string(),
            error_position: 1,
            explanation_ar: "اسم إنَّ منصوب: العلمَ".to_string(),
            explanation_id: "Isim inna harus manshub: العلمَ".to_string(),
            explanation_en: "Noun after inna must be accusative: العلمَ".to_string(),
            options: vec!["العلمَ".to_string(), "العلمُ".to_string(), "العلمِ".to_string()],
        },
        // Kana errors
        SentenceDoctorQuestion {
            id: "sd-012".to_string(),
            incorrect_sentence: "كانَ الجوُّ جميلَ".to_string(),
            correct_sentence: "كانَ الجوُّ جميلاً".to_string(),
            error_type: "irab".to_string(),
            error_position: 2,
            explanation_ar: "خبر كان منصوب، والصحيح: جميلاً (بالفتحة)".to_string(),
            explanation_id: "Khabar kana selalu manshub. Yang benar: جميلاً dengan fathah".to_string(),
            explanation_en: "The predicate of kana is accusative. Correct: جميلاً with fatha".to_string(),
            options: vec!["جميلاً".to_string(), "جميلٌ".to_string(), "جميلٍ".to_string()],
        },
        // Mudaf-Mudaf ilaih
        SentenceDoctorQuestion {
            id: "sd-013".to_string(),
            incorrect_sentence: "كتابُ الطالبُ".to_string(),
            correct_sentence: "كتابُ الطالبِ".to_string(),
            error_type: "irab".to_string(),
            error_position: 1,
            explanation_ar: "المضاف إليه مجرور دائماً، والصحيح: الطالبِ (بالكسرة)".to_string(),
            explanation_id: "Mudhaf ilaih selalu majrur. Yang benar: الطالبِ dengan kasrah".to_string(),
            explanation_en: "The mudaf ilaihi is always genitive. Correct: الطالبِ with kasra".to_string(),
            options: vec!["الطالبِ".to_string(), "الطالبُ".to_string(), "الطالبَ".to_string()],
        },
        SentenceDoctorQuestion {
            id: "sd-014".to_string(),
            incorrect_sentence: "بابُ المسجدُ".to_string(),
            correct_sentence: "بابُ المسجدِ".to_string(),
            error_type: "irab".to_string(),
            error_position: 1,
            explanation_ar: "المضاف إليه مجرور: المسجدِ".to_string(),
            explanation_id: "Mudhaf ilaih harus majrur: المسجدِ".to_string(),
            explanation_en: "Mudaf ilaihi must be genitive: المسجدِ".to_string(),
            options: vec!["المسجدِ".to_string(), "المسجدُ".to_string(), "المسجدَ".to_string()],
        },
        // Na't agreement
        SentenceDoctorQuestion {
            id: "sd-015".to_string(),
            incorrect_sentence: "رأيتُ طالباً مجتهدٌ".to_string(),
            correct_sentence: "رأيتُ طالباً مجتهداً".to_string(),
            error_type: "irab".to_string(),
            error_position: 2,
            explanation_ar: "الصفة تتبع الموصوف في الإعراب. الموصوف منصوب فالصفة منصوبة: مجتهداً".to_string(),
            explanation_id: "Sifat mengikuti mausuf dalam i'rab. Mausuf manshub maka sifat juga manshub: مجتهداً".to_string(),
            explanation_en: "The adjective follows the noun in case. Accusative noun = accusative adjective: مجتهداً".to_string(),
            options: vec!["مجتهداً".to_string(), "مجتهدٌ".to_string(), "مجتهدٍ".to_string()],
        },
    ]
});

/// Get random questions
pub fn get_random_questions(count: usize) -> Vec<SentenceDoctorQuestion> {
    let mut rng = rand::thread_rng();
    let mut questions: Vec<_> = QUESTIONS.iter().cloned().collect();
    questions.shuffle(&mut rng);
    questions.truncate(count);
    questions
}

/// Get questions by level
pub fn get_questions_by_level(level: i32) -> Vec<SentenceDoctorQuestion> {
    if level == 1 {
        QUESTIONS.iter().take(7).cloned().collect()
    } else {
        QUESTIONS.iter().skip(7).cloned().collect()
    }
}
