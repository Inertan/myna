//! Transcription model and language settings.
//!
//! The model list comes from `myna_stt::STT_MODELS` — the same registry the
//! engine loads from — so the dropdown cannot offer a model this build has
//! no decoder for, and cannot offer a language the selected model cannot
//! honour.

use tauri::AppHandle;

use crate::dto::{SttModelDto, TranscriptionPrefsDto};
use crate::error::AppError;
use crate::paths;
use crate::stt_prefs;

/// Lists the ASR models this build can run, with their accepted languages
/// and whether their weights are already on disk.
#[tauri::command]
pub fn list_stt_models(app: AppHandle) -> Result<Vec<SttModelDto>, AppError> {
    let models_root = paths::models_root(&app);

    Ok(myna_stt::STT_MODELS
        .iter()
        .map(|spec| {
            let dir = models_root.join(spec.dir);
            SttModelDto {
                id: spec.id.to_string(),
                label: spec.label.to_string(),
                languages: spec
                    .languages
                    .iter()
                    .map(|language| language.to_string())
                    .collect(),
                language_pin: spec
                    .languages
                    .iter()
                    .any(|language| *language != myna_stt::AUTO_LANGUAGE),
                downloaded: spec
                    .required_files()
                    .iter()
                    .all(|file| dir.join(file).is_file()),
            }
        })
        .collect())
}

/// Returns the persisted transcription settings (model + language).
#[tauri::command]
pub fn get_transcription_prefs(_app: AppHandle) -> Result<TranscriptionPrefsDto, AppError> {
    let root = paths::data_root().map_err(|err| AppError::Path(err.to_string()))?;
    Ok(stt_prefs::load(&root).into())
}

/// Persists transcription settings and returns what was actually stored.
///
/// The pair is normalized first, so an unknown model id or a language the
/// model cannot honour comes back as the default rather than being saved.
/// [`crate::state::AppState::stt_engine`] notices the change on the next
/// acquire and rebuilds the decoder with the newly selected weights.
#[tauri::command]
pub fn set_transcription_prefs(
    _app: AppHandle,
    model: String,
    language: String,
) -> Result<TranscriptionPrefsDto, AppError> {
    let root = paths::data_root().map_err(|err| AppError::Path(err.to_string()))?;
    let normalized = stt_prefs::normalize(&model, &language);
    stt_prefs::save(&root, &normalized)?;
    Ok(normalized.into())
}
