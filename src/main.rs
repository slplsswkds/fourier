mod init;

use std::{
    sync::{Arc, Mutex},
};
use cpal::traits::StreamTrait;
use num_complex::{Complex, ComplexFloat};
use rustfft::{FftPlanner};
use web_view::Content;

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

        visualize_spectre(complex_signal);
    }

    drop(stream);
    
}

#[allow(dead_code)]
fn visualize_spectre(data: Vec<Complex<f32>>) {
    
    println!("visualisation...");
    let len = data.len();
    let mut amplitudes = Vec::new();
    for c in &data[0..len/2] {
        let amplitude = c.abs();
        amplitudes.push(amplitude);
    }
    //------------------------------------------------------------------------------------
    use poloto::build;

    let data: Vec<[i128; 2]> = amplitudes
    .iter()
    .enumerate()
    .map(|(index, &value)| [index as i128, value as i128])
    .collect();

    let a = build::plot("label").line(data);

    let img_svg_src = poloto::frame_build()
        .data(a)
        .build_and_label(("Fourier spectre", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_string().unwrap();

    println!("{}", img_svg_src);

    let mut html_content = "<!DOCTYPE html><html><head><title>SVG Viewer</title></head><body>".to_string();
    html_content = html_content + &img_svg_src + "</body></html>";

    web_view::builder()
        .title("My Project")
        .content(Content::Html(html_content))
        .size(800, 500)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
