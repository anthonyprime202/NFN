use std::collections::VecDeque;

use rodio::Sink;

use crate::models::{PlayerStatus, Song};

#[derive(Default)]
pub struct PlayerState {
    pub status: PlayerStatus,
    pub song: Option<Song>,
}

impl PlayerState {
    pub fn update_song(&mut self, song: Song) {
        self.song = Some(song);
    }
}

pub struct AppState {
    pub sink: Sink,
    pub player: PlayerState,
    pub queue: VecDeque<Song>,
}

impl AppState {
    pub fn new(sink: Sink) -> Self {
        Self {
            sink,
            player: Default::default(),
            queue: Default::default(),
        }
    }
}
