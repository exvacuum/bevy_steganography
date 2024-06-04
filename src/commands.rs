use std::{fs, marker::PhantomData, path::PathBuf};

use bevy::{ecs::system::Command, prelude::*};
use occule::{Codec, CodecError};

use crate::{events::SteganographicFileEvent, resources::StegoCodec};

/// Command which loads a file from a path, attempts to decode it, and broadcasts an event
/// containing decoded data
struct LoadStegoCommand<C>
where
    C: Codec + Send + Sync + 'static,
{
    /// Path of file to decode
    pub path: PathBuf,
    _ph: PhantomData<C>,
}

impl<C> Command for LoadStegoCommand<C>
where
    C: Codec + Send + Sync + 'static,
{
    fn apply(self, world: &mut World) {
        let codec = &world.resource::<StegoCodec<C>>().0;
        let data = fs::read(self.path.clone()).unwrap();
        match codec.decode(&data) {
            Ok((carrier, payload)) => {
                world.send_event(SteganographicFileEvent::<C> {
                    _ph: PhantomData,
                    path: self.path,
                    payload,
                    carrier,
                });
            }
            Err(e) => match e {
                CodecError::DataNotEncoded => {
                    world.send_event(SteganographicFileEvent::<C> {
                        _ph: PhantomData,
                        path: self.path,
                        payload: vec![],
                        carrier: data,
                    });
                }
                _ => warn!("Codec Error: {}", e),
            },
        }
    }
}

impl<C> LoadStegoCommand<C>
where
    C: Codec + Send + Sync + 'static,
{
    fn new(path: PathBuf) -> Self {
        Self {
            _ph: PhantomData,
            path,
        }
    }
}

/// Extension trait to add steganographic loading command
pub trait LoadStegoCommands {
    /// Load and decode a file located at the given path. Results will be broadcast as
    /// `SteganographicFileEvent<C>`s
    fn load_stego<C>(&mut self, path: PathBuf)
    where
        C: Codec + Send + Sync + 'static;
}

impl<'w, 's> LoadStegoCommands for Commands<'w, 's> {
    fn load_stego<C>(&mut self, path: PathBuf)
    where
        C: Codec + Send + Sync + 'static,
    {
        self.add(LoadStegoCommand::<C>::new(path));
    }
}
