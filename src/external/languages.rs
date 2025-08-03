use poise::ChoiceParameter;

#[derive(Debug, Clone, ChoiceParameter)]
pub enum LanguageCode {
    #[name = "English"]
    En,
    #[name = "Spanish"]
    Es,
    #[name = "French"]
    Fr,
    #[name = "German"]
    De,
    #[name = "Chinese (Simplified)"]
    ZhCn,
    #[name = "Chinese (Traditional)"]
    ZhTw,
    #[name = "Japanese"]
    Ja,
    #[name = "Korean"]
    Ko,
    #[name = "Russian"]
    Ru,
    #[name = "Arabic"]
    Ar,
    #[name = "Hindi"]
    Hi,
    #[name = "Portuguese"]
    Pt,
    #[name = "Italian"]
    It,
    #[name = "Dutch"]
    Nl,
    #[name = "Swedish"]
    Sv,
    #[name = "Polish"]
    Pl,
    #[name = "Turkish"]
    Tr,
    #[name = "Hebrew"]
    He,
    #[name = "Vietnamese"]
    Vi,
    #[name = "Indonesian"]
    Id,
    #[name = "Thai"]
    Th,
    #[name = "Bengali"]
    Bn,
    #[name = "Urdu"]
    Ur,
    #[name = "Ukrainian"]
    Uk,
    #[name = "Greek"]
    El,
}

impl LanguageCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageCode::En => "en",
            LanguageCode::Es => "es",
            LanguageCode::Fr => "fr",
            LanguageCode::De => "de",
            LanguageCode::ZhCn => "zh-CN",
            LanguageCode::ZhTw => "zh-TW",
            LanguageCode::Ja => "ja",
            LanguageCode::Ko => "ko",
            LanguageCode::Ru => "ru",
            LanguageCode::Ar => "ar",
            LanguageCode::Hi => "hi",
            LanguageCode::Pt => "pt",
            LanguageCode::It => "it",
            LanguageCode::Nl => "nl",
            LanguageCode::Sv => "sv",
            LanguageCode::Pl => "pl",
            LanguageCode::Tr => "tr",
            LanguageCode::He => "he",
            LanguageCode::Vi => "vi",
            LanguageCode::Id => "id",
            LanguageCode::Th => "th",
            LanguageCode::Bn => "bn",
            LanguageCode::Ur => "ur",
            LanguageCode::Uk => "uk",
            LanguageCode::El => "el",
        }
    }
}
