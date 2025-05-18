use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context};
use tauri::State;

use crate::error::AppResult;
use crate::models::{PlayerStatus, Song};
use crate::state::AppState;
use crate::utils::get_duration;

#[tauri::command]
pub async fn add_to_queue(state: State<'_, Arc<Mutex<AppState>>>, song_name: String) -> AppResult<()> {
    let mut state = state
        .lock()
        .map_err(|e| anyhow!("AppState lock failed: {e}"))?;

    // Getting the file path
    let music_folder = env!("XDG_MUSIC_DIR");
    let home_folder = env!("HOME");
    let path = Path::new(home_folder)
        .join(music_folder)
        .join(format!("{song_name}.mp3"));

    state
        .queue
        .push_back(Song::new(&song_name, get_duration(path).context("Failed to get duration: \n")?));
    Ok(())
}

#[tauri::command]
pub async fn update_status(state: State<'_, Arc<Mutex<AppState>>>, status: PlayerStatus) -> AppResult<()> {
    let mut state = state
        .lock()
        .map_err(|e| anyhow!("AppState lock failed: {e}"))?;
    state.player.status.set_status(status);
    Ok(())
}

