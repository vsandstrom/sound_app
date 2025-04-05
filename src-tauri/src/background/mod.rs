use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Receiver, Sender}, 
    Arc,
    Mutex
  }, 
  thread::JoinHandle,
  time::Duration
};

use cpal::{
  traits::{DeviceTrait, HostTrait, StreamTrait},
  Device,
  Host,
  StreamConfig
};

use rust_dsp::{
  wavetable::shared::Wavetable,
  interpolation::Linear,
  waveshape::triangle,
};

const NUM_AMPS: usize = 16;

type Ctrl = Vec<Receiver<f32>>;

pub struct BackgroundWorker {
  /// Whether the background thread should keep running
  running: Arc<AtomicBool>,
  /// more fields here, e.g. senders and receivers to share data with main thread
  /// Join handle for the background thread
  join: Option<JoinHandle<()>>,
  pub amp_setter: Vec<Sender<f32>>
}

impl Drop for BackgroundWorker {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        self.join.take().unwrap().join().unwrap();
    }
}

impl BackgroundWorker {
  pub fn new() -> Self {
    // create queues for communication
    // let (in_send, in_recv) = mpsc::sync_channel(1024);
    // let (freq_tx, freq_rx) = std::sync::mpsc::channel::<f32>();
    let mut send = Vec::with_capacity(NUM_AMPS);
    let mut recv = Vec::with_capacity(NUM_AMPS);
    for _ in 0..NUM_AMPS {
      let (tx, rx) = channel();
      send.push(tx);
      recv.push(rx);
    };
    
    // spawn bg thread
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let join = std::thread::spawn(move || {
        Self::background_thread(running_clone, Arc::new(Mutex::new(recv as Ctrl)));
    });

    Self {
      join: Some(join),
      running,
      amp_setter: send
    }
  }

  fn background_thread( running: Arc<AtomicBool>, amps: Arc<Mutex<Ctrl>>) {
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
    let mut table = [0.0;2048];
    triangle(&mut table);


    let input_callback = |_data: &[f32], _: &cpal::InputCallbackInfo| { };

    let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| { 
      data
        .chunks_mut(ch)
        .for_each(|frame| {
          let mut sig = 0.0;
          for (i, w) in wt.iter_mut().enumerate() {
            if let Ok(a) = amps.lock().unwrap()[i].try_recv() { temp_amp[i] = a; }
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

