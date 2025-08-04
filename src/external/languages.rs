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
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::De => "de",
            Self::ZhCn => "zh-CN",
            Self::ZhTw => "zh-TW",
            Self::Ja => "ja",
            Self::Ko => "ko",
            Self::Ru => "ru",
            Self::Ar => "ar",
            Self::Hi => "hi",
            Self::Pt => "pt",
            Self::It => "it",
            Self::Nl => "nl",
            Self::Sv => "sv",
            Self::Pl => "pl",
            Self::Tr => "tr",
            Self::He => "he",
            Self::Vi => "vi",
            Self::Id => "id",
            Self::Th => "th",
            Self::Bn => "bn",
            Self::Ur => "ur",
            Self::Uk => "uk",
            Self::El => "el",
        }
    }
}
