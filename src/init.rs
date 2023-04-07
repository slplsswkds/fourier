use cpal::{
    traits::{
        DeviceTrait, 
        HostTrait
    }, 
    Host, 
    Device, 
    StreamConfig, 
    Stream
};
use std::{
    time::Duration, 
    sync::{
        Arc, 
        Mutex
    }
};

fn host_init() -> Host {
    return cpal::default_host();
}

fn input_device_init(host: &Host) -> Device {
    let input_device = match host.default_input_device() {
        Some(dev) => dev,
        None => panic!("noone input device is available"),
    };
    return input_device;
}

fn stream_config(input_device: &Device) -> cpal::StreamConfig {
    let mut stream_config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();
    stream_config.channels = 1u16;
    stream_config.sample_rate = cpal::SampleRate(100000);
    return stream_config;
}

pub fn hw_config(data_storage: Arc<Mutex<Vec<f32>>>) -> (Host, Device, StreamConfig, Stream) {
    let host = host_init();
    let input_device = input_device_init(&host);
    let stream_config = stream_config(&input_device);

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut storage = Vec::new();
        for &sample in data {
            storage.push(sample);
        }
        *data_storage.lock().unwrap() = storage;
    };

    fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }
    
    let stream = input_device.build_input_stream(
        &stream_config, 
        input_data_fn, 
        err_fn, 
        Some(Duration::new(5, 0))).unwrap();

    (host, input_device, stream_config, stream)
} 