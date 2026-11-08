import { Injectable } from '@angular/core';

import type { SttModel, TranscriptionPrefs } from '../../core/models/transcription.model';
import { AUTO_TRANSCRIPTION_LANGUAGE } from '../../core/models/transcription.model';
import { TranscriptionPort } from '../../core/ports/transcription.port';

/** Default model/language pair: the Rust default model with automatic language detection. */
const DEFAULT_PREFS: TranscriptionPrefs = {
  model: 'parakeet-tdt-0.6b-v3-int8',
  language: AUTO_TRANSCRIPTION_LANGUAGE,
};

const DEFAULT_MODELS: readonly SttModel[] = [
  {
    id: 'parakeet-tdt-0.6b-v3-int8',
    label: 'Parakeet-TDT v3',
    languages: [AUTO_TRANSCRIPTION_LANGUAGE],
    languagePin: false,
    downloaded: true,
  },
  {
    id: 'whisper-turbo-int8',
    label: 'Whisper large-v3-turbo',
    languages: [AUTO_TRANSCRIPTION_LANGUAGE, 'en', 'ru'],
    languagePin: true,
    downloaded: false,
  },
];

/** In-memory TranscriptionPort implementation for specs and the placeholder providers. */
@Injectable()
export class InMemoryTranscriptionFake extends TranscriptionPort {
  private models: readonly SttModel[] = DEFAULT_MODELS;
  private prefs: TranscriptionPrefs = DEFAULT_PREFS;

  override async listModels(): Promise<readonly SttModel[]> {
    return this.models;
  }

  override async getPrefs(): Promise<TranscriptionPrefs> {
    return this.prefs;
  }

  /** Mirrors the Rust normalization: unknown model ⇒ default model, unsupported language ⇒ `"auto"`. */
  override async setPrefs(model: string, language: string): Promise<TranscriptionPrefs> {
    const resolved = this.models.find((candidate) => candidate.id === model) ?? this.models[0]!;
    const normalizedLanguage = resolved.languages.includes(language) ? language : AUTO_TRANSCRIPTION_LANGUAGE;
    this.prefs = { model: resolved.id, language: normalizedLanguage };
    return this.prefs;
  }

  /** Test helper: replace the in-memory model registry. */
  seedModels(models: readonly SttModel[]): void {
    this.models = models;
  }

  /** Test helper: seed the persisted model/language pair. */
  seedPrefs(prefs: TranscriptionPrefs): void {
    this.prefs = prefs;
  }
}
