import { TestBed } from '@angular/core/testing';

import { TranscriptionPort } from '../../core/ports/transcription.port';
import { InMemoryTranscriptionFake } from '../testing/in-memory-transcription.fake';
import { GetTranscriptionPrefsUseCase } from './get-transcription-prefs.usecase';

describe('GetTranscriptionPrefsUseCase', () => {
  let useCase: GetTranscriptionPrefsUseCase;
  let transcription: InMemoryTranscriptionFake;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        GetTranscriptionPrefsUseCase,
        InMemoryTranscriptionFake,
        { provide: TranscriptionPort, useExisting: InMemoryTranscriptionFake },
      ],
    });
    useCase = TestBed.inject(GetTranscriptionPrefsUseCase);
    transcription = TestBed.inject(InMemoryTranscriptionFake);
  });

  it('returns the persisted pair exposed by the port', async () => {
    transcription.seedPrefs({ model: 'whisper-turbo-int8', language: 'ru' });

    const prefs = await useCase.get();

    expect(prefs).toEqual({ model: 'whisper-turbo-int8', language: 'ru' });
  });
});
