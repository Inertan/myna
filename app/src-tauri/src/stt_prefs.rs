//! Transcription model and language, persisted to
//! `<data_root>/preferences.json` under the `"transcription"` key this
//! module owns.
//!
//! Mirrors [`crate::summary_prefs`]'s storage contract: reads never fail (a
//! missing or corrupt file yields defaults), and writes merge into the
//! existing top-level object so unrelated keys (`"updates"`, `"summary"`,
//! ...) survive.
//!
//! Validation lives here rather than in the command layer so that *every*
//! read is safe: a stale `model` id from an older build, or a `language`
//! the selected model cannot honour, is normalized on load instead of
//! reaching [`myna_stt::SttEngine`].

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::AppError;
use crate::paths;

const PREFERENCES_FILE: &str = "preferences.json";
const TRANSCRIPTION_KEY: &str = "transcription";

/// Persisted transcription settings.
///
/// `model` is a [`myna_stt::SttModelSpec::id`]; `language` is one of that
/// model's accepted values (always including `"auto"`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptionPrefs {
    pub model: String,
    pub language: String,
}

impl Default for TranscriptionPrefs {
    fn default() -> Self {
        let model = myna_stt::default_model();
        Self {
            model: model.id.to_string(),
            language: myna_stt::AUTO_LANGUAGE.to_string(),
        }
    }
}

/// Coerces a requested `(model, language)` pair into something decodable.
///
/// An unknown model id falls back to the default model; a language the
/// resolved model does not accept falls back to `"auto"`. Both cases are
/// reachable from hand-edited preferences or a downgrade, and neither may
/// reach the decoder.
pub fn normalize(model: &str, language: &str) -> TranscriptionPrefs {
    let spec = myna_stt::model_by_id(model).unwrap_or_else(myna_stt::default_model);
    let language = if spec.accepts_language(language) {
        language
    } else {
        myna_stt::AUTO_LANGUAGE
    };
    TranscriptionPrefs {
        model: spec.id.to_string(),
        language: language.to_string(),
    }
}

/// Loads transcription preferences from `<root>/preferences.json`.
///
/// Never errors: missing, unreadable, corrupt, truncated, or wrongly shaped
/// data all yield [`TranscriptionPrefs::default`] (or a normalized version
/// thereof). A launch must never fail because preferences failed to parse.
pub fn load(root: &Path) -> TranscriptionPrefs {
    let path = root.join(PREFERENCES_FILE);
    let raw = match fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(_) => return TranscriptionPrefs::default(),
    };
    let Ok(root_value) = serde_json::from_str::<Value>(&raw) else {
        return TranscriptionPrefs::default();
    };
    root_value
        .get(TRANSCRIPTION_KEY)
        .cloned()
        .and_then(|value| serde_json::from_value::<TranscriptionPrefs>(value).ok())
        .map(|prefs| normalize(&prefs.model, &prefs.language))
        .unwrap_or_default()
}

/// Persists `prefs` under the `"transcription"` key of
/// `<root>/preferences.json`, preserving unrelated top-level keys. Values
/// are normalized first, so an invalid pair can never be written back.
pub fn save(root: &Path, prefs: &TranscriptionPrefs) -> Result<(), AppError> {
    let path = root.join(PREFERENCES_FILE);
    let normalized = normalize(&prefs.model, &prefs.language);

    let mut root_map = fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .and_then(|value| match value {
            Value::Object(map) => Some(map),
            _ => None,
        })
        .unwrap_or_default();

    let value =
        serde_json::to_value(&normalized).map_err(|err| AppError::Store(err.to_string()))?;
    root_map.insert(TRANSCRIPTION_KEY.to_string(), value);

    paths::create_dir_all_0700(root)?;
    let json = serde_json::to_string_pretty(&Value::Object(root_map))
        .map_err(|err| AppError::Store(err.to_string()))?;
    paths::write_0600(&path, json.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(root: &Path, json: &str) {
        std::fs::write(root.join(PREFERENCES_FILE), json).expect("write preferences");
    }

    #[test]
    fn default_is_the_default_model_with_auto_language() {
        let prefs = TranscriptionPrefs::default();
        assert_eq!(prefs.model, myna_stt::default_model().id);
        assert_eq!(prefs.language, myna_stt::AUTO_LANGUAGE);
    }

    #[test]
    fn load_returns_default_when_the_file_is_missing() {
        let root = tempfile::tempdir().expect("tempdir");
        assert_eq!(load(root.path()), TranscriptionPrefs::default());
    }

    #[test]
    fn load_returns_default_when_the_file_is_corrupt() {
        let root = tempfile::tempdir().expect("tempdir");
        write(root.path(), "{not json");
        assert_eq!(load(root.path()), TranscriptionPrefs::default());
    }

    #[test]
    fn load_round_trips_a_saved_pair() {
        let root = tempfile::tempdir().expect("tempdir");
        let prefs = TranscriptionPrefs {
            model: myna_stt::WHISPER_TURBO.id.to_string(),
            language: "ru".to_string(),
        };

        save(root.path(), &prefs).expect("save");

        assert_eq!(load(root.path()), prefs);
    }

    #[test]
    fn load_normalizes_an_unknown_model_to_the_default() {
        let root = tempfile::tempdir().expect("tempdir");
        write(
            root.path(),
            r#"{"transcription":{"model":"no-such-model","language":"ru"}}"#,
        );

        let prefs = load(root.path());

        assert_eq!(prefs.model, myna_stt::default_model().id);
        // Parakeet has no language pin, so "ru" cannot survive either.
        assert_eq!(prefs.language, myna_stt::AUTO_LANGUAGE);
    }

    #[test]
    fn load_drops_a_language_the_model_cannot_honour() {
        let root = tempfile::tempdir().expect("tempdir");
        write(
            root.path(),
            r#"{"transcription":{"model":"parakeet-tdt-0.6b-v3-int8","language":"ru"}}"#,
        );

        let prefs = load(root.path());

        assert_eq!(prefs.model, myna_stt::PARAKEET_TDT_V3.id);
        assert_eq!(prefs.language, myna_stt::AUTO_LANGUAGE);
    }

    #[test]
    fn save_preserves_unrelated_top_level_keys() {
        let root = tempfile::tempdir().expect("tempdir");
        write(root.path(), r#"{"updates":{"consent":"granted"}}"#);

        save(root.path(), &TranscriptionPrefs::default()).expect("save");

        let raw = std::fs::read_to_string(root.path().join(PREFERENCES_FILE)).expect("read");
        let value: Value = serde_json::from_str(&raw).expect("json");
        assert_eq!(value["updates"]["consent"], "granted");
        assert_eq!(
            value["transcription"]["model"],
            myna_stt::default_model().id
        );
    }

    #[test]
    fn save_normalizes_before_writing() {
        let root = tempfile::tempdir().expect("tempdir");
        let bad = TranscriptionPrefs {
            model: "nope".to_string(),
            language: "xx".to_string(),
        };

        save(root.path(), &bad).expect("save");

        let prefs = load(root.path());
        assert_eq!(prefs.model, myna_stt::default_model().id);
        assert_eq!(prefs.language, myna_stt::AUTO_LANGUAGE);
    }
}
