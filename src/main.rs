mod init;
mod fft;

use std::{
    sync::{Arc, Mutex},
};
use cpal::traits::StreamTrait;
use fft::fft;
use num_complex::Complex;

fn main() {
    let idata = Vec::new();
    let buffer = Arc::new(Mutex::new(idata));
    
    let (_host, 
        _input_device, 
        _stream_config, 
        stream) = init::hw_config(buffer.clone()
    );

    for _times in 0..5 {
        stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(300));

        let complex_signal = buffer.lock().unwrap().iter()
            .map(|&r| Complex::new(r, 0.0)).collect::<Vec<Complex<f32>>>();
        
        fft(complex_signal);
    }

    drop(stream);
    
}
