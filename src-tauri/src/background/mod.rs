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

const NUM_AMPS: usize = 16;

type Ctrl = Vec<Receiver<f32>>;

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
    let join = std::thread::spawn(move || {
        audio_thread::background_thread(running_clone, Arc::new(Mutex::new(recv as Ctrl)));
    });

    Self {
      join: Some(join),
      running,
      amp_setter: send
    }
  }
}
