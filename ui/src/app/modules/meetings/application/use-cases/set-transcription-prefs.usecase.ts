import { Injectable, inject } from '@angular/core';

import type { TranscriptionPrefs } from '../../core/models/transcription.model';
import { TranscriptionPort } from '../../core/ports/transcription.port';

/**
 * Maps onto the frozen Rust command set_transcription_prefs. Resolves the
 * pair the server actually stored: it normalizes an unknown model to the
 * default model and a language the model cannot honour to `"auto"`, so the
 * caller must adopt the result instead of the requested pair.
 */
@Injectable()
export class SetTranscriptionPrefsUseCase {
  private readonly transcription = inject(TranscriptionPort);

  async set(model: string, language: string): Promise<TranscriptionPrefs> {
    return this.transcription.setPrefs(model, language);
  }
}
