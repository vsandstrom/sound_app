mod background;
use background::BackgroundWorker;
use std::sync::Mutex;
use tauri::Manager;
use tauri::State;

#[tauri::command]
fn amp(n: usize, val: f32, state: State<'_, Mutex<BackgroundWorker>>) {
  if let Ok(bg) = state.try_lock() {
    bg.amp_setter[n].send(val).unwrap(); 
  }
}

#[tauri::command]
fn modulation(val: f32, state: State<'_, Mutex<BackgroundWorker>>) {
  if let Ok(bg) = state.try_lock() {
    bg.mod_setter.send(val).unwrap();
  }
}

#[tauri::command]
fn fm(val: f32, state: State<'_, Mutex<BackgroundWorker>>) {
  if let Ok(bg) = state.try_lock() {
    bg.fm_setter.send(val).unwrap();
  }
}

#[tauri::command]
fn fb(val: f32, state: State<'_, Mutex<BackgroundWorker>>) {
  if let Ok(bg) = state.try_lock() {
    bg.fb_setter.send(val).unwrap();
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // create the background worker handling the audio thread
  let bg = BackgroundWorker::new();


  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      // surrender the backtround worker to the app
      app.manage(Mutex::new(bg));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![amp, modulation, fm, fb])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
