mod init;

use std::{
    sync::{Arc, Mutex},
};
use cpal::traits::StreamTrait;
use num_complex::{Complex, ComplexFloat};
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
        println!("recording sample...");
        stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));

        println!("transforming...");
        let mut complex_signal = buffer.lock().unwrap().iter()
            .map(|&r| Complex::new(r, 0.0)).collect::<Vec<Complex<f32>>>();
        
        let fft = planner.plan_fft_forward(complex_signal.len());
        fft.process(&mut complex_signal);

        //visualize_spectre(complex_signal);
    }

    drop(stream);
    
}

#[allow(dead_code)]
fn visualize_spectre(data: Vec<Complex<f32>>) {
    use std::io::Write;
    use std::fs::File;
    
    println!("visualisation...");
    let mut f = File::create("./amplitudes.txt").expect("Unable to create file");

    let len = data.len();
    for c in &data[0..len/2] {
        //let mag = (c.re.powf(2.0).sqrt() + c.im.powf(2.0).sqrt()).sqrt();
        let amplitude = c.abs();
        writeln!(f, "{}", amplitude).unwrap();
    }
}