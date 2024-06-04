use std::{marker::PhantomData, path::PathBuf};

use bevy::prelude::*;
use occule::Codec;

/// Event triggered when a file is decoded using codec C
#[derive(Event)]
pub struct SteganographicFileEvent<C>
where
    C: Codec,
{
    pub(crate) _ph: PhantomData<C>,
    /// Path of file
    pub path: PathBuf,
    /// Carrier data with payload extracted
    pub carrier: Vec<u8>,
    /// Payload data
    pub payload: Vec<u8>,
}
