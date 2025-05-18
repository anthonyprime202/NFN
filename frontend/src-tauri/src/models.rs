use std::{fs::File, io::BufReader, path::Path};

use anyhow::{Context, Result};
use rodio::Decoder;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
pub struct Song {
    pub name: String,
    pub duration: f64,
}

impl Song {
    pub fn new(name: &str, duration: f64) -> Self {
        Self {
            name: name.to_owned(),
            duration,
        }
    }

    pub fn get_source(&self) -> Result<Decoder<BufReader<File>>> {
        let music_folder = env!("XDG_MUSIC_DIR");
        let home_folder = env!("HOME");
        let path = Path::new(home_folder)
            .join(music_folder)
            .join(format!("{}.mp3", self.name));

        let file = BufReader::new(File::open(&path).context("Failed to open music file")?);
        let source = Decoder::new(file).context("Failed to decode music file")?;

        Ok(source)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlayerStatus {
    Playing,
    Stopped,
    Paused,
}

impl PlayerStatus {
    pub fn set_status(&mut self, status: Self) {
        *self = status;
    }
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self::Stopped
    }
}
