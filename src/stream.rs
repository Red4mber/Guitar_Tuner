use std::sync::mpsc;
use cpal::traits::{DeviceTrait, HostTrait};

use crate::fft::{dominant_frequency, SAMPLE_SIZE};


/// Prints a list of available input devices to the console output
pub fn list_input_devices() {
    let host = cpal::default_host();
    println!("Available input devices:");
    for (idx, device) in host.input_devices().unwrap().enumerate() {
        println!("{}: {}", idx, device.name().unwrap());
    }
}


/// Sets up the audio input stream using cpal and the MPSC channels
/// then calls the pitch detection algorithm to find the dominant frequency in the sample
/// before sending it through the MPSC channels to be used in our feedback thread.
pub fn setup_input_stream(device_name: Option<String>) -> (cpal::Stream, mpsc::Receiver<f32>) {
    let host = cpal::default_host();
    let device = if let Some(name) = device_name {
        host.input_devices()
            .unwrap()
            .find(|x| x.name().map_or(false, |n| n == name))
            .expect("Failed to find input device")
    } else {
        host.default_input_device().expect("Failed to get default input device")
    };


    let config = device.default_input_config().unwrap();
    let sample_rate = config.sample_rate().0;


    println!("[^-^] Using input device: {}", device.name().unwrap());
    println!("[^-^] Sample rate : {} Hz", config.sample_rate().0);


    let (tx, rx) = mpsc::channel();
    let mut idx = 0;
    let stream = device.build_input_stream(&config.into(), move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut buffer = vec![0.0; SAMPLE_SIZE];
        for &sample in data {
            if idx == SAMPLE_SIZE  {
                if let Some(freq) = dominant_frequency(&buffer, sample_rate as f32) {
                    tx.send(freq).unwrap();
                }
                idx = 0;
            }
            buffer[idx] = sample;
            idx += 1;
        }
    }, |err| eprintln!("Error in audio stream: {}", err), None).unwrap();

    (stream, rx)
}