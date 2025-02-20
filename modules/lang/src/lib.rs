// Copyright © 2023-2025 andre4ik3
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Launcher Localization Module
//! ============================
//!
//! This crate provides a simple API, [`Language`], for localizing different messages displayed by
//! the launcher. It uses Fluent for localization, and can format messages from a root folder and
//! language identifier.

use fluent_bundle::{FluentArgs, FluentBundle, FluentError, FluentResource};
use fluent_syntax::parser::ParserError;
use std::path::Path;
use thiserror::Error;
use tokio::fs::read_to_string;
use unic_langid::LanguageIdentifier;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read localization resource: {0}")]
    FsError(#[from] std::io::Error),
    #[error("failed to parse localization resource")]
    ParseError { errors: Vec<ParserError> },
    #[error("failed to add localization resource to bundle")]
    AddError { errors: Vec<FluentError> },
}

pub type Result<T> = core::result::Result<T, Error>;

/// The resource files in each language subdirectory that will be loaded into the bundle.
pub const RESOURCES: [&str; 1] = ["main.ftl"];

/// A simple wrapper for localization with Fluent. Each [`Language`] instance represents a wrapped
/// bundle for its specified language.
pub struct Language {
    bundle: FluentBundle<FluentResource>,
}

impl Language {
    /// Attempts to construct a new language bundle from a root path and a language identifier. The
    /// root path should be the path to the `lang` folder, which has subdirectories of each language
    /// identifier (such as `en-US`, `ru-RU`, etc.), which should in turn have the resource files
    /// in [RESOURCES].
    pub async fn try_new(root: impl AsRef<Path>, lang: LanguageIdentifier) -> Result<Self> {
        let root = root.as_ref().join(lang.to_string());
        let mut bundle = FluentBundle::new(vec![lang]);

        // Add each static resource path specified in `RESOURCES`
        for resource in RESOURCES {
            // Construct a path in the form `{root}/{lang}/{resource}`
            let path = root.join(resource);

            // Read it to a string
            let string = read_to_string(path).await?;

            // Parse the string to a resource. Errors aren't usually fatal, but it's best to
            // statically check each resource file and give up loading if it has any errors.
            let resource = FluentResource::try_new(string)
                .map_err(|(_, errors)| Error::ParseError { errors })?;

            // Add the resource to the bundle (will give errors if one or more identifiers conflict)
            bundle
                .add_resource(resource)
                .map_err(|errors| Error::AddError { errors })?;
        }

        Ok(Self { bundle })
    }

    /// Internal common code for [`Language::localize`] and [`Language::format`].
    fn do_format(&self, id: &str, args: Option<&FluentArgs<'_>>) -> String {
        let message = self
            .bundle
            .get_message(id)
            .and_then(|x| x.value())
            .expect("identifier wasn't found in bundle");

        let mut errors = vec![];

        let result = self
            .bundle
            .format_pattern(message, args, &mut errors)
            .to_string();

        if !errors.is_empty() {
            tracing::warn!("Errors occurred whilst localizing `{id}`:");
            for error in errors {
                tracing::warn!("- {}", error);
            }
        }

        result
    }

    /// Localizes a message without any parameters. If errors occur during the localization process,
    /// they are ignored and printed to the console.
    ///
    /// For usage with parameters, see [`Language::format`].
    ///
    /// ## Panics
    ///
    /// Panics if the identifier doesn't exist in the bundle.
    pub fn localize(&self, id: &str) -> String {
        self.do_format(id, None)
    }

    /// Localizes a message with given parameters. If errors occur during the localization process,
    /// they are ignored and printed to the console.
    ///
    /// For usage without parameters, see [`Language::localize`].
    ///
    /// ## Panics
    ///
    /// Panics if the identifier doesn't exist in the bundle.
    pub fn format(&self, id: &str, parameters: &[[&str; 2]]) -> String {
        let mut args = FluentArgs::new();

        for pair in parameters {
            args.set(pair[0], pair[1]);
        }

        self.do_format(id, Some(&args))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use unic_langid::langid;

    async fn language(id: LanguageIdentifier) -> Language {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test");
        Language::try_new(path, id).await.expect("Failed to create language")
    }

    #[tokio::test]
    async fn localize_english() {
        let language = language(langid!("en-US")).await;
        let value = language.localize("localize-example");
        assert_eq!(value, "This is a string without parameters.");
    }

    #[tokio::test]
    #[should_panic(expected = "identifier wasn't found in bundle")]
    async fn localize_english_nonexistent() {
        let language = language(langid!("en-US")).await;
        let value = language.localize("localize-example-nonexistent");
        assert!(value.is_empty()); // Placeholder; call above should panic
    }

    #[tokio::test]
    async fn format_english() {
        let language = language(langid!("en-US")).await;

        let value = language.format(
            "format-example",
            &[["greeting", "Hello"], ["target", "world"]],
        );

        // \u{2068} and \u{2069} are control characters to support instances in which there is text
        // of both directions in the same string (e.g. embedded RTL text in LTR string). \u{2068}
        // marks the beginning of a directional segment and \u{2069} marks the end. The direction of
        // the segment is determined by the first "strongly directional" character of the segment.
        assert_eq!(value, "\u{2068}Hello\u{2069}, \u{2068}world\u{2069}! This is a string with parameters.");

        let value = language.format(
            "format-example",
            &[["greeting", "Hi"], ["target", "dear user"]],
        );

        assert_eq!(value, "\u{2068}Hi\u{2069}, \u{2068}dear user\u{2069}! This is a string with parameters.");
    }

    #[tokio::test]
    #[should_panic(expected = "identifier wasn't found in bundle")]
    async fn format_english_nonexistent() {
        let language = language(langid!("en-US")).await;

        let value = language.format(
            "format-example-nonexistent",
            &[["greeting", "Hello"], ["target", "world"]],
        );

        assert!(value.is_empty()); // Placeholder; call above should panic
    }

    #[tokio::test]
    async fn localize_russian() {
        let language = language(langid!("ru-RU")).await;
        let value = language.localize("localize-example");
        assert_eq!(value, "Пример строки без параметров.");
    }

    #[tokio::test]
    async fn format_russian() {
        let language = language(langid!("ru-RU")).await;

        let value = language.format(
            "format-example",
            &[["greeting", "Привет"], ["target", "мир"]],
        );

        assert_eq!(value, "\u{2068}Привет\u{2069}, \u{2068}мир\u{2069}! Это пример строки с параметрами.");

        let value = language.format(
            "format-example",
            &[["greeting", "Здравствуй"], ["target", "пользователь"]],
        );

        assert_eq!(value, "\u{2068}Здравствуй\u{2069}, \u{2068}пользователь\u{2069}! Это пример строки с параметрами.");
    }
}
