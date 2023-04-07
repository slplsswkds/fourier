mod init;

use std::{
    sync::{Arc, Mutex},
};
use cpal::traits::StreamTrait;
use num_complex::Complex;
use rustfft::{FftPlanner};


fn main() {
    let idata = Vec::new();
    let buffer = Arc::new(Mutex::new(idata));
    
    let (_host, 
        _input_device, 
        _stream_config, 
        stream) = init::hw_config(buffer.clone()
    );

    let mut planner = FftPlanner::new();

    for _times in 0..1 {
        stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(300));

        let mut complex_signal = buffer.lock().unwrap().iter()
            .map(|&r| Complex::new(r, 0.0)).collect::<Vec<Complex<f32>>>();
        
        // Shoud be factorization
        if complex_signal.len() % 2 == 0 {} else {
            complex_signal.pop();
        }

        let fft = planner.plan_fft_forward(complex_signal.len());
        fft.process(&mut complex_signal);

        for val in complex_signal {
            println!("{}", val);
        }
    }

    drop(stream);
    
}
