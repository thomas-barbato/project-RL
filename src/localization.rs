use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextCatalog {
    locales: BTreeMap<String, BTreeMap<String, String>>,
}

impl TextCatalog {
    pub fn register(
        &mut self,
        locale: String,
        key: String,
        value: String,
    ) -> Result<(), TextCatalogError> {
        if locale.trim().is_empty() {
            return Err(TextCatalogError::EmptyLocale);
        }
        if key.trim().is_empty() {
            return Err(TextCatalogError::EmptyKey);
        }
        if value.trim().is_empty() {
            return Err(TextCatalogError::EmptyValue { locale, key });
        }

        let entries = self.locales.entry(locale.clone()).or_default();
        if entries.contains_key(&key) {
            return Err(TextCatalogError::DuplicateKey { locale, key });
        }
        entries.insert(key, value);
        Ok(())
    }

    pub fn resolve<'a>(&'a self, locale: &str, key: &str) -> Option<&'a str> {
        self.locales
            .get(locale)
            .and_then(|entries| entries.get(key))
            .map(String::as_str)
    }

    pub fn locale_count(&self) -> usize {
        self.locales.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextCatalogError {
    EmptyLocale,
    EmptyKey,
    EmptyValue { locale: String, key: String },
    DuplicateKey { locale: String, key: String },
}

impl Display for TextCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyLocale => write!(formatter, "text locale must not be empty"),
            Self::EmptyKey => write!(formatter, "text key must not be empty"),
            Self::EmptyValue { locale, key } => {
                write!(
                    formatter,
                    "text '{key}' in locale '{locale}' must not be empty"
                )
            }
            Self::DuplicateKey { locale, key } => {
                write!(formatter, "duplicate text key '{key}' in locale '{locale}'")
            }
        }
    }
}

impl Error for TextCatalogError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_are_resolved_by_locale_and_duplicate_keys_are_rejected() {
        let mut catalog = TextCatalog::default();
        assert_eq!(
            catalog.register(
                "fr".to_owned(),
                "technique.scan.name".to_owned(),
                "Analyse".to_owned(),
            ),
            Ok(())
        );
        assert_eq!(
            catalog.resolve("fr", "technique.scan.name"),
            Some("Analyse")
        );
        assert_eq!(catalog.resolve("en", "technique.scan.name"), None);
        assert!(matches!(
            catalog.register(
                "fr".to_owned(),
                "technique.scan.name".to_owned(),
                "Autre".to_owned(),
            ),
            Err(TextCatalogError::DuplicateKey { .. })
        ));
    }
}
