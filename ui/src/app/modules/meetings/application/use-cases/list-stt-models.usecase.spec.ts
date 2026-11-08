import { TestBed } from '@angular/core/testing';

import { TranscriptionPort } from '../../core/ports/transcription.port';
import { InMemoryTranscriptionFake } from '../testing/in-memory-transcription.fake';
import { ListSttModelsUseCase } from './list-stt-models.usecase';

describe('ListSttModelsUseCase', () => {
  let useCase: ListSttModelsUseCase;
  let transcription: InMemoryTranscriptionFake;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        ListSttModelsUseCase,
        InMemoryTranscriptionFake,
        { provide: TranscriptionPort, useExisting: InMemoryTranscriptionFake },
      ],
    });
    useCase = TestBed.inject(ListSttModelsUseCase);
    transcription = TestBed.inject(InMemoryTranscriptionFake);
  });

  it('returns the models exposed by the port, not a hardcoded list', async () => {
    transcription.seedModels([
      {
        id: 'whisper-turbo-int8',
        label: 'Whisper',
        languages: ['auto', 'en'],
        languagePin: true,
        downloaded: true,
      },
    ]);

    const models = await useCase.list();

    expect(models).toEqual([
      {
        id: 'whisper-turbo-int8',
        label: 'Whisper',
        languages: ['auto', 'en'],
        languagePin: true,
        downloaded: true,
      },
    ]);
  });

  it('returns the default in-memory models when nothing is seeded', async () => {
    const models = await useCase.list();

    expect(models.length).toBeGreaterThan(0);
  });
});
