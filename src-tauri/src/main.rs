#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{Manager, Window, Monitor, PhysicalPosition};
mod aura_memory;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  rgb: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {

            let app_handle = app.handle();
            let window = app.get_window("main").unwrap();
            let monitors = window.available_monitors().unwrap();
            let memory = aura_memory::Memory::new();


            for monitor in monitors {
                let monitor_size = monitor.size();
                if monitor_size.width == 1024 {
                    let position = monitor.position();
                    window.set_position(*position);
                }
            }


            // Set up a new thread, get RGB values from LightingService.exe from memory
            std::thread::spawn(move || {
                loop {
                    let rgb = memory.get();

                    // Send to window
                    app_handle.emit_all("rgb", Payload { rgb: rgb }).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

