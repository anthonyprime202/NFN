mod commands;
mod error;
mod models;
mod state;
mod utils;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Result;
use models::PlayerStatus;
use rodio::{OutputStream, Sink};
use state::AppState;
use tauri::{Emitter, Manager};
use tokio::time::interval;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let app_state = Arc::new(Mutex::new(AppState::new(sink)));
            let app_handle = app.handle().clone();
            let state_clone = app_state.clone();

            tauri::async_runtime::spawn(async move {
                let mut ticker = interval(Duration::from_millis(500));
                loop {
                    ticker.tick().await;
                    let mut state = state_clone.lock().unwrap();
                    let AppState {
                        player,
                        sink,
                        queue,
                    } = &mut *state;

                    match player.status {
                        PlayerStatus::Playing => {
                            if sink.empty() {
                                if let Some(song) = queue.pop_front() {
                                    // When sink is empty and there is a song in the queue
                                    if let Ok(source) = song.get_source() {
                                        sink.append(source);
                                        player.update_song(song);
                                        let _ = app_handle.emit("player-appended", &player.song);
                                    }
                                } else {
                                    // When sink is empty and there is no song in queue
                                    let _ = app_handle.emit("player-ended", ());
                                }
                            } else if sink.is_paused() {
                                sink.play();
                            } else {
                                // When sink is not empty
                                let _ = app_handle.emit("player-progress", sink.get_pos().as_secs_f64());
                            }
                        }
                        PlayerStatus::Paused => {
                            if !sink.is_paused() {
                                sink.pause();
                            }
                        }
                        PlayerStatus::Stopped => {}
                    }

                    // Add to sink when sink is empty and player is not playing
                    // Update progress bar when player is playing
                    // When sink is empty and player is playing mark emit the completion event
                }
            });

            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::update_status,
            commands::add_to_queue
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
