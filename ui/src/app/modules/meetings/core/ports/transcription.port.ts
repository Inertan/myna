import type { SttModel, TranscriptionPrefs } from '../models/transcription.model';

/**
 * Maps onto the frozen Rust commands list_stt_models,
 * get_transcription_prefs, and set_transcription_prefs.
 */
export abstract class TranscriptionPort {
  /** The Rust-owned model registry, in dropdown order — never a hardcoded second copy. */
  abstract listModels(): Promise<readonly SttModel[]>;
  /** The persisted model/language pair; missing or invalid data is normalized Rust-side. */
  abstract getPrefs(): Promise<TranscriptionPrefs>;
  /**
   * Persists `(model, language)` and resolves the pair actually stored.
   * The server may fall back — an unknown model becomes the default model,
   * and a language the model cannot honour becomes `"auto"` — so callers
   * must adopt the returned pair rather than assume the request stuck.
   */
  abstract setPrefs(model: string, language: string): Promise<TranscriptionPrefs>;
}
