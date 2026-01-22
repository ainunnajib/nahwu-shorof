use std::collections::HashMap;
use std::sync::LazyLock;

type TranslationMap = HashMap<&'static str, HashMap<&'static str, &'static str>>;

static TRANSLATIONS: LazyLock<TranslationMap> = LazyLock::new(|| {
    let mut m: TranslationMap = HashMap::new();

    // App
    m.insert("app.name", [("id", "Afkaaruna"), ("ar", "أفكارنا"), ("en", "Afkaaruna")].into());
    m.insert("app.tagline", [
        ("id", "Belajar Nahwu & Shorof dengan Menyenangkan"),
        ("ar", "تعلم النحو والصرف بطريقة ممتعة"),
        ("en", "Learn Nahwu & Shorof the Fun Way"),
    ].into());

    // Navigation
    m.insert("nav.home", [("id", "Beranda"), ("ar", "الرئيسية"), ("en", "Home")].into());
    m.insert("nav.play", [("id", "Bermain"), ("ar", "العب"), ("en", "Play")].into());
    m.insert("nav.learn", [("id", "Belajar"), ("ar", "تعلم"), ("en", "Learn")].into());
    m.insert("nav.profile", [("id", "Profil"), ("ar", "الملف الشخصي"), ("en", "Profile")].into());

    // Game UI
    m.insert("game.start", [("id", "Mulai"), ("ar", "ابدأ"), ("en", "Start")].into());
    m.insert("game.continue", [("id", "Lanjutkan"), ("ar", "استمر"), ("en", "Continue")].into());
    m.insert("game.retry", [("id", "Coba Lagi"), ("ar", "حاول مرة أخرى"), ("en", "Try Again")].into());
    m.insert("game.next", [("id", "Selanjutnya"), ("ar", "التالي"), ("en", "Next")].into());
    m.insert("game.finish", [("id", "Selesai"), ("ar", "انتهى"), ("en", "Finish")].into());
    m.insert("game.score", [("id", "Skor"), ("ar", "النتيجة"), ("en", "Score")].into());
    m.insert("game.correct", [("id", "Benar!"), ("ar", "صحيح!"), ("en", "Correct!")].into());
    m.insert("game.incorrect", [("id", "Kurang Tepat"), ("ar", "غير صحيح"), ("en", "Incorrect")].into());
    m.insert("game.excellent", [("id", "Luar Biasa!"), ("ar", "ممتاز!"), ("en", "Excellent!")].into());

    // Game Names
    m.insert("game.wordBuilder", [("id", "Pembangun Kata"), ("ar", "بناء الكلمات"), ("en", "Word Builder")].into());
    m.insert("game.wordBuilder.desc", [
        ("id", "Bangun kata dari akar huruf dan pola wazan"),
        ("ar", "ابنِ كلمات من الجذور والأوزان"),
        ("en", "Build words from roots and patterns"),
    ].into());
    m.insert("game.sentenceDoctor", [("id", "Dokter Kalimat"), ("ar", "طبيب الجمل"), ("en", "Sentence Doctor")].into());
    m.insert("game.sentenceDoctor.desc", [
        ("id", "Temukan dan perbaiki kesalahan i'rab dalam kalimat"),
        ("ar", "صحّح أخطاء الإعراب في الجمل"),
        ("en", "Find and fix i'rab errors in sentences"),
    ].into());
    m.insert("game.grammarMatch", [("id", "Cocokkan Tata Bahasa"), ("ar", "مطابقة القواعد"), ("en", "Grammar Match")].into());
    m.insert("game.conjugationRace", [("id", "Lomba Tashrif"), ("ar", "سباق التصريف"), ("en", "Conjugation Race")].into());

    // Topics
    m.insert("topic.nahwu", [("id", "Nahwu"), ("ar", "النحو"), ("en", "Nahwu")].into());
    m.insert("topic.nahwu.desc", [
        ("id", "Ilmu tentang susunan kalimat Arab"),
        ("ar", "علم تركيب الجمل العربية"),
        ("en", "The science of Arabic sentence structure"),
    ].into());
    m.insert("topic.shorof", [("id", "Shorof"), ("ar", "الصرف"), ("en", "Shorof")].into());
    m.insert("topic.shorof.desc", [
        ("id", "Ilmu tentang perubahan bentuk kata"),
        ("ar", "علم تغيير صيغة الكلمات"),
        ("en", "The science of word morphology"),
    ].into());

    // Profile
    m.insert("profile.level", [("id", "Level"), ("ar", "المستوى"), ("en", "Level")].into());
    m.insert("profile.totalXP", [("id", "Total XP"), ("ar", "مجموع نقاط الخبرة"), ("en", "Total XP")].into());
    m.insert("profile.achievements", [("id", "Pencapaian"), ("ar", "الإنجازات"), ("en", "Achievements")].into());
    m.insert("profile.streak", [("id", "Hari Berturut"), ("ar", "أيام متتالية"), ("en", "Day Streak")].into());

    // Buttons
    m.insert("button.submit", [("id", "Kirim"), ("ar", "إرسال"), ("en", "Submit")].into());
    m.insert("button.check", [("id", "Periksa"), ("ar", "تحقق"), ("en", "Check")].into());
    m.insert("button.hint", [("id", "Petunjuk"), ("ar", "تلميح"), ("en", "Hint")].into());
    m.insert("button.skip", [("id", "Lewati"), ("ar", "تخطي"), ("en", "Skip")].into());
    m.insert("button.back", [("id", "Kembali"), ("ar", "رجوع"), ("en", "Back")].into());

    // Instructions
    m.insert("instruction.wordBuilder", [
        ("id", "Susun huruf sesuai pola yang diberikan untuk membentuk kata"),
        ("ar", "رتّب الحروف حسب الوزن المعطى لتكوين الكلمة"),
        ("en", "Arrange the letters according to the pattern to form the word"),
    ].into());
    m.insert("instruction.sentenceDoctor", [
        ("id", "Temukan dan perbaiki kesalahan dalam kalimat berikut"),
        ("ar", "اعثر على الخطأ في الجملة التالية وصححه"),
        ("en", "Find and fix the error in the following sentence"),
    ].into());

    // Results
    m.insert("result.title", [("id", "Hasil Permainan"), ("ar", "نتيجة اللعبة"), ("en", "Game Results")].into());
    m.insert("result.score", [("id", "Skor Anda"), ("ar", "نتيجتك"), ("en", "Your Score")].into());
    m.insert("result.stars", [("id", "Bintang"), ("ar", "النجوم"), ("en", "Stars")].into());
    m.insert("result.xp", [("id", "XP Diperoleh"), ("ar", "نقاط الخبرة"), ("en", "XP Earned")].into());

    // Misc
    m.insert("loading", [("id", "Memuat..."), ("ar", "جاري التحميل..."), ("en", "Loading...")].into());
    m.insert("error.generic", [("id", "Terjadi kesalahan"), ("ar", "حدث خطأ"), ("en", "An error occurred")].into());

    m
});

/// Get translation for a key and language
pub fn t(key: &str, lang: &str) -> &'static str {
    TRANSLATIONS
        .get(key)
        .and_then(|m| m.get(lang).or_else(|| m.get("en")))
        .copied()
        .unwrap_or(key)
}
