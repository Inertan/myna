import { TestBed } from '@angular/core/testing';

import type { SummaryLanguage } from '../../../core/models/summary-language.model';
import type { SttModel } from '../../../core/models/transcription.model';
import { SettingsComponent } from './settings.component';

const summaryLanguages: readonly SummaryLanguage[] = [
  { code: 'en', label: 'English' },
  { code: 'fr', label: 'French' },
];

const parakeet: SttModel = {
  id: 'parakeet-tdt-0.6b-v3-int8',
  label: 'Parakeet-TDT v3',
  languages: ['auto'],
  languagePin: false,
  downloaded: true,
};
const whisper: SttModel = {
  id: 'whisper-turbo-int8',
  label: 'Whisper large-v3-turbo',
  languages: ['auto', 'en', 'ru'],
  languagePin: true,
  downloaded: false,
};
const sttModels: readonly SttModel[] = [parakeet, whisper];

describe('SettingsComponent transcription section', () => {
  const createFixture = (models: readonly SttModel[] = sttModels, selectedModel = parakeet.id) => {
    const fixture = TestBed.createComponent(SettingsComponent);
    fixture.componentRef.setInput('updateConsent', 'granted');
    fixture.componentRef.setInput('summaryLanguages', summaryLanguages);
    fixture.componentRef.setInput('selectedSummaryLanguage', 'en');
    fixture.componentRef.setInput('transcriptionModels', models);
    fixture.componentRef.setInput('selectedTranscriptionModel', selectedModel);
    fixture.componentRef.setInput('selectedTranscriptionLanguage', 'auto');
    fixture.detectChanges();
    return fixture;
  };

  it('renders one option per model and marks the selected one', () => {
    const fixture = createFixture(sttModels, whisper.id);

    const select: HTMLSelectElement = fixture.nativeElement.querySelector('.transcription-model-select');
    expect(Array.from(select.options).map((option) => option.value)).toEqual([parakeet.id, whisper.id]);
    expect(select.value).toBe(whisper.id);
  });

  it('disables the model picker until the Rust-owned list has loaded', () => {
    const fixture = createFixture([]);

    expect(fixture.nativeElement.querySelector('.transcription-model-select').disabled).toBe(true);
  });

  it('disables the language picker for a model with no language pin', () => {
    const fixture = createFixture();

    expect(fixture.nativeElement.querySelector('.transcription-language-select').disabled).toBe(true);
  });

  it("enables the language picker and lists that model's languages when a pin exists", () => {
    const fixture = createFixture(sttModels, whisper.id);

    const select: HTMLSelectElement = fixture.nativeElement.querySelector('.transcription-language-select');
    expect(select.disabled).toBe(false);
    expect(Array.from(select.options).map((option) => option.value)).toEqual(['auto', 'en', 'ru']);
  });

  it('shows a visible hint when the selected model is not downloaded yet', () => {
    const downloaded = createFixture();
    expect(downloaded.nativeElement.querySelector('.transcription-not-downloaded')).toBeNull();

    const pending = createFixture(sttModels, whisper.id);
    expect(pending.nativeElement.querySelector('.transcription-not-downloaded')?.textContent).toContain('downloaded yet');
  });

  it('emits the chosen model id', () => {
    const fixture = createFixture();
    const emitted: string[] = [];
    fixture.componentInstance.transcriptionModelSelected.subscribe((id) => emitted.push(id));

    const select: HTMLSelectElement = fixture.nativeElement.querySelector('.transcription-model-select');
    select.value = whisper.id;
    select.dispatchEvent(new Event('change'));

    expect(emitted).toEqual([whisper.id]);
  });

  it('emits the chosen language code', () => {
    const fixture = createFixture(sttModels, whisper.id);
    const emitted: string[] = [];
    fixture.componentInstance.transcriptionLanguageSelected.subscribe((code) => emitted.push(code));

    const select: HTMLSelectElement = fixture.nativeElement.querySelector('.transcription-language-select');
    select.value = 'ru';
    select.dispatchEvent(new Event('change'));

    expect(emitted).toEqual(['ru']);
  });
});
