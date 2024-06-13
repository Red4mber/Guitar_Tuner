use rustfft::{Fft, FftPlanner};
use std::f64::consts::PI;
use rustfft::num_complex::Complex;


// pub const SAMPLE_SIZE: usize = 16384;
const NUM_HPS: usize = 6; // Max number of harmonic product spectrums

pub const SAMPLE_SIZE: usize = 8192; // Window size of the DFT in samples
const WHITE_NOISE_THRESH: f32 = 0.1; // Threshold for noise suppression


/// Finds the dominant frequency of a sample using the HPS algorithm.
///
/// Massively overcomplicated because I'm having consistency issues, so I tried various methods to clean up the results
/// I'm a shit programmer so of course it still doesn't work... TODO Fix this shit, future-me
pub fn dominant_frequency(data: &[f32], sample_rate: f32) -> Option<f32> {
    // Setup FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(SAMPLE_SIZE);

    // Convert our samples to Complex numbers
    let buffer: Vec<Complex<f32>> = data.iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect();

    // Create a janky-ass Blackman Window
    let mut windowed_samples = buffer.to_vec();
    let window: Vec<f32> = (0..SAMPLE_SIZE)
        .map(|i| 0.5 * (1.0 - (-2.0 * PI as f32 * (i as f32) / (SAMPLE_SIZE as f32 - 1.0)).cos()))
        .collect();
    // Apply the window to our sample for improved FFT accuracy
    for i in 0..SAMPLE_SIZE {
        windowed_samples[i] *= window[i];
    }

    // Simple FFT
    let mut fft_input = windowed_samples.into_iter().collect::<Vec<_>>();
    fft.process(&mut fft_input);

    // De-Complexinator
    let mut hps_spec = fft_input
        .iter()
        .take(SAMPLE_SIZE)
        .map(|c| c.norm())
        .collect::<Vec<_>>();

    // Suppress low frequencies
    for i in 0..(50.0 / (sample_rate / SAMPLE_SIZE as f32)).floor() as usize {
        hps_spec[i] = 0.0;
    }

    // Suppress high frequencies
    for i in (6000.0 / (sample_rate / SAMPLE_SIZE as f32)).ceil() as usize..hps_spec.len() {
        hps_spec[i] = 0.0;
    }

    // Octave-band noise suppression
    let mut octave_bands = vec![];
    let mut start = 0;
    let mut end = 0;
    let mut band_center_freq = 31.25;
    while band_center_freq < sample_rate / 2.0 {
        end = (band_center_freq * 2.0 / (sample_rate / SAMPLE_SIZE as f32)).ceil() as usize;
        octave_bands.push((start, end));
        start = end;
        band_center_freq *= 2.0;
    }

    for (start, end) in octave_bands.iter() {
        let avg_energy_per_freq = hps_spec[*start..*end]
            .iter()
            .map(|x| x * x)
            .sum::<f32>()
            / (*end - *start) as f32;
        let avg_energy_per_freq = avg_energy_per_freq.sqrt();

        for i in *start..*end {
            hps_spec[i] = if hps_spec[i] > (WHITE_NOISE_THRESH * avg_energy_per_freq) {
                hps_spec[i]
            } else {
                0.0
            };
        }
    }

    for i in NUM_HPS..0 {
        let mut tmp_hps_spec = Vec::with_capacity(hps_spec.len() / (i + 1));
        for j in (0..hps_spec.len()).step_by(i + 1) {
            let a = hps_spec[j];
            let b = fft_input[j].norm();
            tmp_hps_spec.push(a * b);
        }
        hps_spec = tmp_hps_spec;
    }

    // Returns the index of the highest magnitude bin
    let max_idx = hps_spec
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap();

    // Convert it to a usable frequency
    let max_freq = (max_idx as f32 * sample_rate / SAMPLE_SIZE as f32) / NUM_HPS as f32;
    if max_freq > 20.0 && max_freq < 6000.0 {
        Some(max_freq)
    } else {
        None
    }
}