/**
 * A speech-to-text model offered by the Rust core. `id` is the stable
 * identifier persisted in preferences and sent back over IPC; `languages`
 * is that model's accepted language list, always starting with `"auto"`.
 * The Rust core owns the authoritative registry — this module never
 * hardcodes a second copy of it.
 */
export interface SttModel {
  readonly id: string;
  readonly label: string;
  readonly languages: readonly string[];
  /** `false` when the model ignores language entirely (Parakeet) — the language picker must be disabled. */
  readonly languagePin: boolean;
  /** `false` while the model's weights are not on disk yet — still selectable, but flagged in the UI. */
  readonly downloaded: boolean;
}

/** The persisted transcription model/language pair returned by `get_transcription_prefs` / `set_transcription_prefs`. */
export interface TranscriptionPrefs {
  readonly model: string;
  readonly language: string;
}

/**
 * Fallback model id used until a real preference has been loaded. The Rust
 * server normalizes an empty or unknown id to its own default model, so
 * this is never stored as a real selection.
 */
export const DEFAULT_TRANSCRIPTION_MODEL_ID = '';

/**
 * Language value meaning "let the model decide". Mirrors the Rust
 * `AUTO_LANGUAGE` constant — always `languages[0]`, and the only accepted
 * value for a model with `languagePin: false`.
 */
export const AUTO_TRANSCRIPTION_LANGUAGE = 'auto';

/** Fallback language used until a real preference has been loaded; mirrors the Rust server's own fallback. */
export const DEFAULT_TRANSCRIPTION_LANGUAGE_CODE = AUTO_TRANSCRIPTION_LANGUAGE;
