import { Injectable, inject } from '@angular/core';

import type { TranscriptionPrefs } from '../../core/models/transcription.model';
import { TranscriptionPort } from '../../core/ports/transcription.port';

/** Maps onto the frozen Rust command get_transcription_prefs. */
@Injectable()
export class GetTranscriptionPrefsUseCase {
  private readonly transcription = inject(TranscriptionPort);

  async get(): Promise<TranscriptionPrefs> {
    return this.transcription.getPrefs();
  }
}
