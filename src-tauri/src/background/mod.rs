mod audio_thread;

use std::{
  sync::{ atomic::{AtomicBool, Ordering}, Arc }, 
  thread::JoinHandle,
};

use parking_lot::Mutex;
use crossbeam_channel::{unbounded, Receiver, Sender};

use audio_thread::audio_process;

const NUM_OSC: usize = 16;
const NUM_FILT: usize = 12;

/// Modify this to match the control parameters that the background worker
/// should be able to listen to.
///
/// Example: 
/// ```
/// const NUM = 8
/// // `send` will be accessable from outer process
/// let mut send = Vec::with_capacity(NUM);
/// // `recv` will be passed to subprocess 
/// let mut recv = Vec::with_capacity(NUM);
/// // `send_filter` will be accessable from outer process
/// let mut send_filter = Vec::with_capacity(NUM);
/// // `recv_filter` will be passed to subprocess 
/// let mut recv_filter = Vec::with_capacity(NUM);
/// for _ in 0..NUM {
///   let (tx, rx) = unbounded(); 
///   send.push(tx); 
///   recv.push(rx); 
///   let (tx, rx) = unbounded(); 
///   send_filter.push(tx); 
///   recv_filter.push(rx); 
/// };
/// let (mod_tx, mod_rx) = unbounded();
/// let (fm_tx, fm_rx) = unbounded();
/// let (fb_tx, fb_rx) = unbounded();
/// 
/// // ready to be passed to another thread
/// let ctrl = Arc::new(Mutex::new(Ctrl{amps: recv, filter: recv_filter, modulation: mod_rx, fm: fm_rx, fb: fb_rx}));
/// ```
struct Ctrl {
  amps: Vec<Receiver<f32>>,
  filter: Vec<Receiver<f32>>,
  modulation: Receiver<f32>,
  fm: Receiver<f32>,
  fb: Receiver<f32>,
  mix: Receiver<f32>
}

// type Ctrl = Vec<Receiver<f32>>;

pub struct BackgroundWorker {
  running: Arc<AtomicBool>,
  join: Option<JoinHandle<()>>,
  pub amp_setter: Vec<Sender<f32>>,
  pub filter_setter: Vec<Sender<f32>>,
  pub mod_setter: Sender<f32>,
  pub fm_setter: Sender<f32>,
  pub fb_setter: Sender<f32>,
  pub mix_setter: Sender<f32>
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
    let mut send = Vec::with_capacity(NUM_OSC);
    let mut recv = Vec::with_capacity(NUM_OSC);
    let mut send_filt = Vec::with_capacity(NUM_FILT);
    let mut recv_filt = Vec::with_capacity(NUM_FILT);
    for _ in 0..NUM_OSC {
      let (tx, rx) = unbounded();
      send.push(tx);
      recv.push(rx);
    };

    for _ in 0..(NUM_FILT) {
      let (tx, rx) = unbounded();
      send_filt.push(tx);
      recv_filt.push(rx);
    }

    let (mod_tx, mod_rx) = unbounded();
    let (fm_tx, fm_rx) = unbounded();
    let (fb_tx, fb_rx) = unbounded();
    let (mix_tx, mix_rx) = unbounded();
    
    // spawn bg thread
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let ctrl = Arc::new(Mutex::new(Ctrl{amps: recv, filter: recv_filt, modulation: mod_rx, fm: fm_rx, fb: fb_rx, mix: mix_rx}));
    let join = std::thread::spawn(move || {
        audio_process(running_clone, ctrl);
    });

    Self {
      join: Some(join),
      running,
      amp_setter: send,
      filter_setter: send_filt,
      mod_setter: mod_tx,
      fm_setter: fm_tx,
      fb_setter: fb_tx,
      mix_setter: mix_tx
      
    }
  }
}
