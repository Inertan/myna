import { TestBed } from '@angular/core/testing';

import { TranscriptionPort } from '../../core/ports/transcription.port';
import { InMemoryTranscriptionFake } from '../testing/in-memory-transcription.fake';
import { SetTranscriptionPrefsUseCase } from './set-transcription-prefs.usecase';

describe('SetTranscriptionPrefsUseCase', () => {
  let useCase: SetTranscriptionPrefsUseCase;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        SetTranscriptionPrefsUseCase,
        InMemoryTranscriptionFake,
        { provide: TranscriptionPort, useExisting: InMemoryTranscriptionFake },
      ],
    });
    useCase = TestBed.inject(SetTranscriptionPrefsUseCase);
  });

  it('forwards the requested model and language and resolves the stored pair', async () => {
    const prefs = await useCase.set('whisper-turbo-int8', 'ru');

    expect(prefs).toEqual({ model: 'whisper-turbo-int8', language: 'ru' });
  });

  it('adopts the server fallback when an unknown model is requested', async () => {
    const prefs = await useCase.set('no-such-model', 'ru');

    expect(prefs).toEqual({ model: 'parakeet-tdt-0.6b-v3-int8', language: 'auto' });
  });

  it('adopts the server fallback when the model cannot honour the language', async () => {
    const prefs = await useCase.set('parakeet-tdt-0.6b-v3-int8', 'ru');

    expect(prefs).toEqual({ model: 'parakeet-tdt-0.6b-v3-int8', language: 'auto' });
  });
});
