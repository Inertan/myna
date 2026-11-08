//! Model-gated end-to-end decode through a registry model.
//!
//! Not part of the normal suite — it needs real weights on disk and an
//! audio file, and it prints a transcript rather than asserting on one, so
//! it doubles as the A/B harness for comparing models on the same audio.
//!
//! ```text
//! MYNA_STT_MODEL_DIR=/path/to/models/parakeet-tdt-0.6b-v3-int8 \
//! MYNA_STT_WAV=/path/to/audio.wav \
//! cargo test -p myna-stt --test registry_decode -- --ignored --nocapture
//! ```
//!
//! Optional environment:
//! - `MYNA_STT_MODEL_ID` — registry entry to use; defaults to the entry
//!   whose `dir` matches the model directory's name, else the default model.
//! - `MYNA_STT_LANGUAGE` — language pin (e.g. `ru`); only meaningful for
//!   models that accept one.

use std::path::PathBuf;

use myna_stt::{model_by_dir, model_by_id, read_wav_to_f32, SttConfig, SttEngine};

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).map(PathBuf::from)
}

#[test]
#[ignore = "model-gated: needs MYNA_STT_MODEL_DIR + MYNA_STT_WAV pointing at real artifacts"]
fn transcribes_a_wav_with_the_selected_registry_model() {
    let Some(model_dir) = env_path("MYNA_STT_MODEL_DIR") else {
        return;
    };
    let Some(wav) = env_path("MYNA_STT_WAV") else {
        return;
    };

    let spec = std::env::var("MYNA_STT_MODEL_ID")
        .ok()
        .as_deref()
        .and_then(model_by_id)
        .or_else(|| {
            model_dir
                .file_name()
                .and_then(|name| name.to_str())
                .and_then(model_by_dir)
        })
        .unwrap_or_else(myna_stt::default_model);

    let language = std::env::var("MYNA_STT_LANGUAGE").ok();

    let engine = SttEngine::load(&SttConfig {
        model_dir,
        spec,
        language: language
            .as_deref()
            .and_then(|requested| spec.decode_language(requested)),
        ..SttConfig::default()
    })
    .expect("load the selected model");

    // Chunked like production: the streamer never hands a decoder more than
    // `MAX_DECODE_CHUNK_SEC`, and Whisper additionally discards anything past
    // its own 30 s limit. Whole-file decode would therefore measure the
    // harness, not the model.
    let chunk_sec: f32 = std::env::var("MYNA_STT_CHUNK_SEC")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(5.0);

    let (samples, sample_rate) = read_wav_to_f32(&wav).expect("read wav");
    let chunk_len = ((sample_rate as f32 * chunk_sec) as usize).max(1);

    let mut text = String::new();
    for chunk in samples.chunks(chunk_len) {
        let piece = engine
            .transcribe_samples(sample_rate as i32, chunk)
            .expect("transcribe chunk");
        if !piece.trim().is_empty() {
            text.push_str(piece.trim());
            text.push('\n');
        }
    }

    println!(
        "--- {} (language: {}, chunk {}s) ---",
        spec.id,
        language.as_deref().unwrap_or("auto"),
        chunk_sec
    );
    println!("{text}");
}
