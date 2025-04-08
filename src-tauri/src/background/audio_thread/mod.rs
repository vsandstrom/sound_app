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
  wavetable::shared::Wavetable,
  noise::Noise,
  dsp::signal::map
};

use super::Ctrl;

const NUM_OSC: usize = 16;
const NUM_PARTS: [u32; 4] = [1, 3, 5, 7];
const VOL_PARTS: [f32; 4] = [1.0, 0.85, 0.725, 0.5];

pub fn audio_process( running: Arc<AtomicBool>, ctrl: Arc<Mutex<Ctrl>>) {
  let host = cpal::default_host();
  let (input_device, output_device, config) = match get_io(&host) {
    Ok((i, o, c)) => (i, o, c),
    Err(e) => panic!("{}", e)
  };
  let samplerate = config.sample_rate.0 as f32;
  let ch = config.channels as usize;

  let mut temp_amp = [0.0; NUM_OSC];
  let mut temp_mod = 0.0;
  let mut temp_fb = 0.0;
  let mut temp_fm = 0.0;

  //
  // Audio Setup
  //

  let mut wt: [Wavetable; NUM_OSC*4] = std::array::from_fn(|_| Wavetable::new());
  let mut nz: [Noise; NUM_OSC] = std::array::from_fn(|_| Noise::new(samplerate));
  wt.iter_mut().for_each(|w| w.set_samplerate(samplerate));
  let freq: [f32; NUM_OSC] = std::array::from_fn(|i| (i as f32 + 1.0) * 45.0);
  let table = [0.0;2048].triangle();

  let input_callback = |_data: &[f32], _: &cpal::InputCallbackInfo| { };

  let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| { 
    data
      .chunks_mut(ch)
      .for_each(|frame| {

        if let Ok(c) = ctrl.lock() {
            temp_mod = c.modulation.try_recv().unwrap_or(temp_mod);
            temp_fm = c.fm.try_recv().unwrap_or(temp_fm);
            temp_fb = c.fb.try_recv().unwrap_or(temp_fb);
        } 

        let modulation = nz
          .iter_mut()
          .map(|n| 
            map(&mut n.play(1.0/12.0), -1.0, 1.0, 0.0, 1.0) * temp_mod
          ).collect::<Vec<f32>>();
        let mut sig: f32 = 0.0;
        for (i, w) in wt.chunks_mut(4).enumerate() {
          if let Ok(a) = ctrl.lock().unwrap().amps[i].try_recv() { temp_amp[i] = a; }
          sig += w.iter_mut()
            .enumerate()
            .fold(0.0,|acc, (j, x)| {
              acc + x.play::<Linear>(
                &table,
                freq[i]*NUM_PARTS[j] as f32,
                modulation[i] * 0.004 + acc / (j+1) as f32 * temp_fm) * temp_amp[i] * VOL_PARTS[j]}
              )
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
