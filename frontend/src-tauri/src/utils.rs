use std::fs::File;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use symphonia::{
    core::{io::MediaSourceStream, probe::Hint},
    default::get_probe,
};

pub fn get_duration(path: PathBuf) -> Result<f64> {
    // Getting the source file
    let file = File::open(&path).context("Failed to open music file")?;
    let source = MediaSourceStream::new(Box::new(file), Default::default());

    // Creating a hint
    let mut hint = Hint::new();
    hint.with_extension(".mp3");

    // Getting the source format
    let format = get_probe()
        .format(&hint, source, &Default::default(), &Default::default())
        .context("Failed to get format of the source music file")?
        .format;

    // Creating a track
    let track = format
        .default_track()
        .ok_or_else(|| anyhow!("Failed to create a track from music source"))?;

    let n_frames = track
        .codec_params
        .n_frames
        .ok_or_else(|| anyhow!("N Frame not found in media source"))? as f64;

    let sample_rate = track
        .codec_params
        .sample_rate
        .ok_or_else(|| anyhow!("Sample frames not found in media source"))?
        as f64;

    Ok(n_frames / sample_rate)
}
