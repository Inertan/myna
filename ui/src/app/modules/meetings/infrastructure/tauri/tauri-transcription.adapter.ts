import { Injectable } from '@angular/core';

import type { SttModel, TranscriptionPrefs } from '../../core/models/transcription.model';
import { TranscriptionPort } from '../../core/ports/transcription.port';
import { mapSttModelDtoToDomain, mapTranscriptionPrefsDtoToDomain } from '../mappers/transcription.mapper';
import { invokeCommand } from './ipc';

/** `TranscriptionPort` implementation backed by the Tauri IPC command surface. */
@Injectable()
export class TauriTranscriptionAdapter extends TranscriptionPort {
  override async listModels(): Promise<readonly SttModel[]> {
    const dtos = await invokeCommand('list_stt_models', {});
    return dtos.map(mapSttModelDtoToDomain);
  }

  override async getPrefs(): Promise<TranscriptionPrefs> {
    return mapTranscriptionPrefsDtoToDomain(await invokeCommand('get_transcription_prefs', {}));
  }

  override async setPrefs(model: string, language: string): Promise<TranscriptionPrefs> {
    const dto = await invokeCommand('set_transcription_prefs', { model, language });
    return mapTranscriptionPrefsDtoToDomain(dto);
  }
}
