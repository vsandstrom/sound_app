mod audio_thread;

use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Receiver, Sender}, 
    Arc,
    Mutex
  }, 
  thread::JoinHandle,
};

use audio_thread::audio_process;

const NUM_AMPS: usize = 16;

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
/// for _ in 0..NUM {
///   let (tx, rx) = channel(); 
///   send.push(tx); 
///   recv.push(rx); 
/// };
/// 
/// // ready to be passed to another thread
/// let ctrl = Arc::new(Mutex::new(Ctrl{amps: recv}));
/// ```
struct Ctrl {
  amps: Vec<Receiver<f32>>
}

// type Ctrl = Vec<Receiver<f32>>;

pub struct BackgroundWorker {
  running: Arc<AtomicBool>,
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
    let ctrl = Arc::new(Mutex::new(Ctrl{amps: recv}));
    let join = std::thread::spawn(move || {
        audio_process(running_clone, ctrl);
    });

    Self {
      join: Some(join),
      running,
      amp_setter: send
    }
  }
}
