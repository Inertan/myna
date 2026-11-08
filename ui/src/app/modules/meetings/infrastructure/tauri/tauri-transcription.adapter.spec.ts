import { TestBed } from '@angular/core/testing';

import { installTauriInternalsStub, uninstallTauriInternalsStub } from './testing/tauri-internals.stub';
import { TauriTranscriptionAdapter } from './tauri-transcription.adapter';

describe('TauriTranscriptionAdapter', () => {
  let adapter: TauriTranscriptionAdapter;

  beforeEach(() => {
    TestBed.configureTestingModule({ providers: [TauriTranscriptionAdapter] });
    adapter = TestBed.inject(TauriTranscriptionAdapter);
  });

  afterEach(() => uninstallTauriInternalsStub());

  it('listModels() invokes list_stt_models and maps every SttModelDto to the domain shape', async () => {
    let receivedCmd: string | undefined;
    installTauriInternalsStub((cmd) => {
      receivedCmd = cmd;
      return [
        {
          id: 'parakeet-tdt-0.6b-v3-int8',
          label: 'Parakeet',
          languages: ['auto'],
          languagePin: false,
          downloaded: true,
        },
      ];
    });

    const models = await adapter.listModels();

    expect(receivedCmd).toBe('list_stt_models');
    expect(models).toEqual([
      {
        id: 'parakeet-tdt-0.6b-v3-int8',
        label: 'Parakeet',
        languages: ['auto'],
        languagePin: false,
        downloaded: true,
      },
    ]);
  });

  it('getPrefs() invokes get_transcription_prefs and maps the returned pair', async () => {
    let receivedCmd: string | undefined;
    installTauriInternalsStub((cmd) => {
      receivedCmd = cmd;
      return { model: 'whisper-turbo-int8', language: 'ru' };
    });

    const prefs = await adapter.getPrefs();

    expect(receivedCmd).toBe('get_transcription_prefs');
    expect(prefs).toEqual({ model: 'whisper-turbo-int8', language: 'ru' });
  });

  it('setPrefs() invokes set_transcription_prefs with camelCase args and resolves the normalized pair', async () => {
    let receivedCmd: string | undefined;
    let receivedArgs: unknown;
    installTauriInternalsStub((cmd, args) => {
      receivedCmd = cmd;
      receivedArgs = args;
      return { model: 'parakeet-tdt-0.6b-v3-int8', language: 'auto' };
    });

    const prefs = await adapter.setPrefs('parakeet-tdt-0.6b-v3-int8', 'ru');

    expect(receivedCmd).toBe('set_transcription_prefs');
    expect(receivedArgs).toEqual({ model: 'parakeet-tdt-0.6b-v3-int8', language: 'ru' });
    expect(prefs).toEqual({ model: 'parakeet-tdt-0.6b-v3-int8', language: 'auto' });
  });
});
