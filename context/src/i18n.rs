use std::collections::HashMap;

use fluent_templates::{fs::langid, static_loader, LanguageIdentifier};

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");

static_loader! {
    pub static LOCALES = {
        locales: "../locales",
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

lazy_static! {
    pub static ref LANGUAGE_MAP: HashMap<String, LanguageIdentifier> = {
        let mut m = HashMap::new();

        m.insert("en-US".to_string(), US_ENGLISH);

        m
    };
}
