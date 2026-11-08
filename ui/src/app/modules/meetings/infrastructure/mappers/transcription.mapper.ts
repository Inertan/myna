import type { SttModel, TranscriptionPrefs } from '../../core/models/transcription.model';
import type { SttModelDto, TranscriptionPrefsDto } from '../dto/transcription.dto';

/** Maps a `SttModelDto` entry (from `list_stt_models`) to the domain `SttModel`. */
export function mapSttModelDtoToDomain(dto: SttModelDto): SttModel {
  return {
    id: dto.id,
    label: dto.label,
    languages: dto.languages,
    languagePin: dto.languagePin,
    downloaded: dto.downloaded,
  };
}

/** Maps a `TranscriptionPrefsDto` to the domain `TranscriptionPrefs`. */
export function mapTranscriptionPrefsDtoToDomain(dto: TranscriptionPrefsDto): TranscriptionPrefs {
  return { model: dto.model, language: dto.language };
}
