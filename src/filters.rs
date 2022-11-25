// This module provides enums that can be used as filters for articles
use serde::{Serialize, Deserialize};

/// Perigon classifies news articles into 13 categories
#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    Politics,
    Tech,
    Sports,
    Business,
    Finance,
    Entertainment,
    Health,
    Weather,
    Lifestyle,
    Auto,
    Science,
    Travel,
    Environment,
    None,           //  for uncategorized
}


/// Articles can be filtered by country
#[derive(Serialize, Deserialize, Debug)]
pub enum Country {
    US, // United States 🇺🇲
    GB, // Great Britain 🇬🇧
    DE, // Germany 🇩🇪
    IT, // Italy 🇮🇹
    FR, // France 🇫🇷
    CA, // Canada 🇨🇦
    NL, // Netherlands 🇳🇱
    SE, // Sweden 🇸🇪
    DK, // Denmark 🇩🇰
    FI, // Finland 🇫🇮
    HU, // Hungary 🇭🇺
    NO, // Norway 🇳🇴
    PL, // Poland 🇵🇱
    PT, // Portugal 🇵🇹
    RU, // Russia 🇷🇺
    UA, // Ukraine 🇺🇦
    CH, // Switzerland 🇨🇭
    BR, // Brazil 🇧🇷
    NZ, // New Zealand 🇳🇿
    MX, // Mexico 🇲🇽
    AU, // Australia 🇦🇺
}


/// Articles can be filtered by language
#[derive(Serialize, Deserialize, Debug)]
pub enum Language {
    EN, // English
    DE, // German
    IT, // Italian
    FR, // Franench
    NL, // Dutch
    SV, // Swedish
    DA, // Danish
    FI, // Finnish
    HU, // Hungarian
    NO, // Norwegian
    PL, // Polish
    PT, // Portugese
    RU, // Russiam
    UK, // Ukrainian
    ES, // Spanish
}


