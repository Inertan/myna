/**
 * Wire shapes for transcription settings crossing the Tauri IPC boundary.
 * Both mirror Rust structs with `#[serde(rename_all = "camelCase")]`.
 */

/** Mirrors one entry of the Rust `list_stt_models` result (`SttModelDto`). */
export interface SttModelDto {
  readonly id: string;
  readonly label: string;
  readonly languages: readonly string[];
  readonly languagePin: boolean;
  readonly downloaded: boolean;
}

/** Mirrors the Rust `TranscriptionPrefsDto` — the persisted model/language pair. */
export interface TranscriptionPrefsDto {
  readonly model: string;
  readonly language: string;
}
