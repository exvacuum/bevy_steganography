use std::path::PathBuf;

use bevy::prelude::*;

/// Event triggered when a file is decoded
#[derive(Event)]
pub struct SteganographicFileEvent
{
    /// Path of file
    pub path: PathBuf,
    /// Carrier data with payload extracted
    pub carrier: Option<Vec<u8>>,
    /// Payload data
    pub payload: Option<Vec<u8>>,
}
