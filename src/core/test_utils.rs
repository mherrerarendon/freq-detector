use float_cmp::ApproxEq;

use serde::Deserialize;
use std::fs;

use crate::{core::utils::sine_wave_signal, frequency::FrequencyDetector};

use super::utils::audio_buffer_to_signal;

#[derive(Deserialize)]
pub struct SampleData {
    pub data: Option<Vec<u8>>,
}

pub fn test_signal(filename: &str) -> anyhow::Result<Vec<f64>> {
    let file_path = format!("{}/test_data/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let mut sample_data: SampleData = serde_json::from_str(&fs::read_to_string(&file_path)?)?;
    let buffer = sample_data.data.take().unwrap();
    Ok(audio_buffer_to_signal(&buffer))
}

pub fn test_fundamental_freq<D: FrequencyDetector>(
    detector: &mut D,
    samples_file: &str,
    expected_freq: f64,
) -> anyhow::Result<()> {
    pub const TEST_SAMPLE_RATE: f64 = 44000.0;
    let signal = test_signal(samples_file)?;

    let freq = detector
        .detect_frequency(&signal, TEST_SAMPLE_RATE)
        .ok_or(anyhow::anyhow!("Did not get pitch"))?;

    assert!(
        freq.approx_eq(expected_freq, (0.02, 2)),
        "Expected freq: {}, Actual freq: {}",
        expected_freq,
        freq
    );
    Ok(())
}

pub fn test_sine_wave<D: FrequencyDetector>(detector: &mut D, freq: f64) -> anyhow::Result<()> {
    const SAMPLE_RATE: f64 = 44100.0;
    let signal = sine_wave_signal(8192, 440., SAMPLE_RATE);

    let actual_freq = detector
        .detect_frequency(&signal, SAMPLE_RATE)
        .ok_or(anyhow::anyhow!("Did not get pitch"))?;

    assert!(
        actual_freq.approx_eq(freq, (0.1, 1)),
        "Expected freq: {}, Actual freq: {}",
        freq,
        actual_freq
    );
    Ok(())
}
