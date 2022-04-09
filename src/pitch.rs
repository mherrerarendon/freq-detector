pub mod cepstrum;
pub mod core;
pub mod hanned_fft;

// autocorrelation doesn't work well enough yet.
// pub mod autocorrelation;

use std::ops::Range;

use crate::core::{utils::interpolated_peak_at, FftPoint};

pub trait PitchDetector: SignalToSpectrum {
    fn detect_pitch(
        &mut self,
        signal: &[f64],
        sample_rate: f64,
        freq_range_hint: Option<Range<f64>>,
    ) -> Option<f64> {
        let (start_bin, spectrum) =
            self.signal_to_spectrum(signal, freq_range_hint.map(|r| (r, sample_rate)));
        let max_bin =
            spectrum.iter().enumerate().reduce(
                |accum, item| {
                    if item.1 > accum.1 {
                        item
                    } else {
                        accum
                    }
                },
            )?;

        let FftPoint { x: bin, .. } = interpolated_peak_at(&spectrum, max_bin.0)?;
        Some(self.bin_to_freq(bin + start_bin as f64, sample_rate))
    }
}

pub trait SignalToSpectrum {
    fn signal_to_spectrum(
        &mut self,
        signal: &[f64],
        freq_range: Option<(Range<f64>, f64)>,
    ) -> (usize, Vec<f64>);

    // Bin may be float resolution
    fn bin_to_freq(&self, bin: f64, sample_rate: f64) -> f64;
    fn freq_to_bin(&self, freq: f64, sample_rate: f64) -> f64;

    fn name(&self) -> &'static str;
}
