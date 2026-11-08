//! Registry of the offline ASR models this app can run.
//!
//! One entry per downloadable model: what sherpa-onnx family it belongs to,
//! which files it ships, and which output languages it can be pinned to.
//! The engine ([`crate::engine`]) and the app's model downloader
//! (`app/src-tauri/src/paths.rs`) both read this list, so a model is
//! added in exactly one place.
//!
//! ## Why language is per-model
//!
//! Parakeet-TDT v3 has **no language pin**: it decodes greedily over one
//! joint vocabulary shared by 25 languages. A short or noisy segment can
//! therefore settle into the wrong language and stay there — the
//! Russian→English drift this module exists to give users a way out of.
//! Whisper, by contrast, takes an explicit `language` and forces it. That
//! is why [`SttModelSpec::languages`] is a per-model list rather than a
//! global one: offering "Russian" on Parakeet would promise something the
//! model cannot do.

/// Which sherpa-onnx offline model family a registry entry uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SttModelKind {
    /// NeMo transducer: encoder + decoder + joiner, no language pin.
    Transducer,
    /// OpenAI Whisper: encoder + decoder, optional language pin.
    Whisper,
}

/// Language value meaning "let the model decide".
///
/// For Whisper this becomes `None` (its own language detection); for a
/// model with no pin it is the only accepted value.
pub const AUTO_LANGUAGE: &str = "auto";

/// A supported offline ASR model.
///
/// `dir` is the directory name under the app's models root; the file names
/// are relative to it.
#[derive(Debug, Clone, Copy)]
pub struct SttModelSpec {
    /// Stable identifier persisted in preferences and passed over IPC.
    pub id: &'static str,
    /// Human-readable label shown in the settings dropdown.
    pub label: &'static str,
    pub kind: SttModelKind,
    /// Directory under the models root.
    pub dir: &'static str,
    pub encoder: &'static str,
    pub decoder: &'static str,
    /// `Some(_)` for transducer models only.
    pub joiner: Option<&'static str>,
    pub tokens: &'static str,
    /// Accepted `language` values, always including [`AUTO_LANGUAGE`].
    pub languages: &'static [&'static str],
}

impl SttModelSpec {
    /// Every artifact this model needs on disk.
    pub fn required_files(&self) -> Vec<&'static str> {
        let mut files = vec![self.encoder, self.decoder, self.tokens];
        if let Some(joiner) = self.joiner {
            files.push(joiner);
        }
        files
    }

    /// Whether `language` may be persisted for this model.
    ///
    /// Unknown values are rejected by callers rather than silently stored,
    /// so a typo in preferences cannot reach the decoder.
    pub fn accepts_language(&self, language: &str) -> bool {
        self.languages.contains(&language)
    }

    /// Longest audio window, in seconds, this model may be asked to decode
    /// in one call.
    ///
    /// The two families fail in opposite directions: Parakeet-TDT drifts
    /// into English once a window exceeds ~5 s (see `stream.rs`), while
    /// Whisper discards everything past 30 s and hallucinates subtitle
    /// boilerplate on short, silence-heavy windows. One global cap cannot
    /// serve both, so the window travels with the model.
    pub fn decode_window_sec(&self) -> f32 {
        match self.kind {
            // Shared with the streamer so the two cannot drift apart.
            SttModelKind::Transducer => crate::stream::MAX_DECODE_CHUNK_SEC,
            SttModelKind::Whisper => 25.0,
        }
    }

    /// The language to hand sherpa-onnx: `None` for [`AUTO_LANGUAGE`].
    pub fn decode_language(&self, language: &str) -> Option<String> {
        if language == AUTO_LANGUAGE {
            None
        } else {
            Some(language.to_string())
        }
    }
}

/// Parakeet-TDT v3 (int8). Multilingual, but with no language pin — see the
/// module docs.
pub const PARAKEET_TDT_V3: SttModelSpec = SttModelSpec {
    id: "parakeet-tdt-0.6b-v3-int8",
    label: "Parakeet-TDT v3 — 25 languages, no language pin",
    kind: SttModelKind::Transducer,
    dir: "parakeet-tdt-0.6b-v3-int8",
    encoder: "encoder.int8.onnx",
    decoder: "decoder.int8.onnx",
    joiner: Some("joiner.int8.onnx"),
    tokens: "tokens.txt",
    languages: &[AUTO_LANGUAGE],
};

/// Whisper large-v3-turbo (int8). Slower than Parakeet, but it takes an
/// explicit language, which is what makes a language picker meaningful.
pub const WHISPER_TURBO: SttModelSpec = SttModelSpec {
    id: "whisper-turbo-int8",
    label: "Whisper large-v3-turbo — explicit language",
    kind: SttModelKind::Whisper,
    dir: "sherpa-onnx-whisper-turbo",
    encoder: "turbo-encoder.int8.onnx",
    decoder: "turbo-decoder.int8.onnx",
    joiner: None,
    tokens: "turbo-tokens.txt",
    languages: &[
        AUTO_LANGUAGE,
        "en",
        "ru",
        "uk",
        "de",
        "fr",
        "es",
        "it",
        "pt",
        "nl",
        "pl",
        "tr",
        "cs",
        "sv",
        "da",
        "fi",
        "ro",
        "hu",
        "el",
        "bg",
        "hr",
        "sk",
        "sl",
        "et",
        "lv",
        "lt",
    ],
};

/// Every model the app may use, in dropdown order.
pub const STT_MODELS: &[SttModelSpec] = &[PARAKEET_TDT_V3, WHISPER_TURBO];

/// The model used when preferences are missing, corrupt, or name an
/// unknown id.
///
/// Parakeet stays the default: it is the model existing installs already
/// have on disk, and picking Whisper by default would silently force a
/// ~800 MB download on the next recording.
pub fn default_model() -> &'static SttModelSpec {
    &PARAKEET_TDT_V3
}

/// Looks up a model by its stable id.
pub fn model_by_id(id: &str) -> Option<&'static SttModelSpec> {
    STT_MODELS.iter().find(|model| model.id == id)
}

/// Looks up a model by the directory name it occupies under the models root.
pub fn model_by_dir(dir: &str) -> Option<&'static SttModelSpec> {
    STT_MODELS.iter().find(|model| model.dir == dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_and_dirs_are_unique() {
        for (index, model) in STT_MODELS.iter().enumerate() {
            for other in &STT_MODELS[index + 1..] {
                assert_ne!(model.id, other.id, "duplicate model id");
                assert_ne!(model.dir, other.dir, "duplicate model dir");
            }
        }
    }

    #[test]
    fn every_model_accepts_auto_language() {
        for model in STT_MODELS {
            assert!(
                model.accepts_language(AUTO_LANGUAGE),
                "{} must accept {AUTO_LANGUAGE}",
                model.id
            );
        }
    }

    #[test]
    fn transducer_models_require_a_joiner_and_whisper_does_not() {
        for model in STT_MODELS {
            assert_eq!(
                model.kind == SttModelKind::Transducer,
                model.joiner.is_some(),
                "{}: joiner presence must match the model family",
                model.id
            );
        }
    }

    #[test]
    fn a_model_without_a_language_pin_offers_only_auto() {
        // Guards the promise the settings dropdown makes: Parakeet cannot
        // honour "ru", so it must not be listed as accepting it.
        assert_eq!(PARAKEET_TDT_V3.languages, &[AUTO_LANGUAGE]);
    }

    #[test]
    fn decode_language_maps_auto_to_none() {
        assert_eq!(WHISPER_TURBO.decode_language(AUTO_LANGUAGE), None);
        assert_eq!(WHISPER_TURBO.decode_language("ru"), Some("ru".to_string()));
    }

    #[test]
    fn whisper_gets_a_longer_decode_window_than_the_transducer() {
        // Whisper discards audio past 30 s and needs the context that short
        // windows destroy; Parakeet drifts on anything over ~5 s.
        assert!(WHISPER_TURBO.decode_window_sec() <= 30.0);
        assert!(WHISPER_TURBO.decode_window_sec() > PARAKEET_TDT_V3.decode_window_sec());
        assert_eq!(PARAKEET_TDT_V3.decode_window_sec(), 5.0);
    }

    #[test]
    fn default_model_is_in_the_registry() {
        let default = default_model();
        assert!(STT_MODELS.iter().any(|model| model.id == default.id));
    }

    #[test]
    fn lookup_by_id_and_dir_round_trips() {
        for model in STT_MODELS {
            assert_eq!(model_by_id(model.id).map(|m| m.id), Some(model.id));
            assert_eq!(model_by_dir(model.dir).map(|m| m.id), Some(model.id));
        }
    }
}
