use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
    Mutex
  }, 
  time::Duration
};

use cpal::{
  traits::{DeviceTrait, HostTrait, StreamTrait},
  Device,
  Host,
  StreamConfig
};

use rust_dsp::{
  interpolation::Linear, 
  waveshape::traits::Waveshape,
  wavetable::shared::Wavetable
};

use super::Ctrl;

const NUM_AMPS: usize = 16;

pub fn audio_process( running: Arc<AtomicBool>, ctrl: Arc<Mutex<Ctrl>>) {
  let host = cpal::default_host();
  let (input_device, output_device, config) = match get_io(&host) {
    Ok((i, o, c)) => (i, o, c),
    Err(e) => panic!("{}", e)
  };
  let samplerate = config.sample_rate.0 as f32;
  let ch = config.channels as usize;

  let mut temp_amp = [0.0; NUM_AMPS];

  //
  // Audio Setup
  //

  let mut wt: [Wavetable; NUM_AMPS] = std::array::from_fn(|_| Wavetable::new());
  wt.iter_mut().for_each(|w| w.set_samplerate(samplerate));
  let freq: [f32; NUM_AMPS] = std::array::from_fn(|i| (i as f32 + 1.0) * 45.0);
  let table = [0.0;2048].triangle();

  let input_callback = |_data: &[f32], _: &cpal::InputCallbackInfo| { };

  let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| { 
    data
      .chunks_mut(ch)
      .for_each(|frame| {
        let mut sig = 0.0;
        for (i, w) in wt.iter_mut().enumerate() {
          if let Ok(a) = ctrl.lock().unwrap().amps[i].try_recv() { temp_amp[i] = a; }
          sig += w.play::<Linear>(&table, freq[i], 0.0) * temp_amp[i];
        }

      frame.iter_mut().for_each(|sample| *sample = sig.tanh());
    });
  };

  let err_callback = |err: cpal::StreamError| { eprintln!("{}", err) };

  let input_stream = input_device.build_input_stream(
      &config, 
      input_callback,
      err_callback,
      None
  ).expect("unable to build input stream");

  let output_stream = output_device.build_output_stream(
      &config,
      output_callback,
      err_callback,
      None
  ).expect("unable to build output stream");

  input_stream.play().expect("unable to init play input stream");
  output_stream.play().expect("unable to init play output stream");
  

  while running.load(Ordering::Relaxed) {
    std::thread::sleep(Duration::new(1, 0));
    // keep alive
  }
}

fn get_io(host: &Host) -> Result<(Device, Device, StreamConfig), Box<dyn std::error::Error>>{
  let i_dev = match host.default_input_device() {
    Some(dev) => dev,
    None => return Err("no input device".into())
  };

  let o_dev = match host.default_output_device() {
    Some(dev) => dev,
    None => return Err("no output device".into())
  };

  let config = i_dev.default_input_config()?;
  Ok((i_dev, o_dev, config.into()))
}
