import { Injectable, inject } from '@angular/core';

import type { SttModel } from '../../core/models/transcription.model';
import { TranscriptionPort } from '../../core/ports/transcription.port';

/** Maps onto the frozen Rust command list_stt_models. */
@Injectable()
export class ListSttModelsUseCase {
  private readonly transcription = inject(TranscriptionPort);

  async list(): Promise<readonly SttModel[]> {
    return this.transcription.listModels();
  }
}
