use std::{fs, path::PathBuf};

use bevy::{ecs::system::Command, prelude::*};
use occule::CodecError;

use crate::{events::SteganographicFileEvent, resources::StegoCodecs};

/// Command which loads a file from a path, attempts to decode it, and broadcasts an event
/// containing decoded data
struct LoadStegoCommand {
    /// Path of file to decode
    pub path: PathBuf,
}

impl Command for LoadStegoCommand {
    fn apply(self, world: &mut World) {
        let codecs = world.remove_resource::<StegoCodecs>().unwrap();
        let mut processed = false;
        let mut data = None;
        if let Some(extension) = self.path.extension() {
            let extension = extension.to_string_lossy().to_string();
            data = Some(fs::read(self.path.clone()).unwrap());
            let data = data.as_ref().unwrap();
            for (k, codec) in codecs.0.iter().rev() {
                if k.contains(&extension) {
                    match codec.decode(data) {
                        Ok((carrier, payload)) => {
                            world.send_event(SteganographicFileEvent {
                                path: self.path.clone(),
                                payload: Some(payload),
                                carrier: Some(carrier),
                            });
                            processed = true;
                            break;
                        }
                        Err(e) => match e {
                            CodecError::DataNotEncoded => {}
                            _ => warn!("Codec Error: {}", e),
                        },
                    }
                }
            }
        }
        if !processed {
            world.send_event(SteganographicFileEvent {
                path: self.path,
                payload: None,
                carrier: data,
            });
        }
        world.insert_resource(codecs);
    }
}

impl LoadStegoCommand {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

/// Extension trait to add steganographic loading command
pub trait LoadStegoCommands {
    /// Load and decode a file located at the given path. Results will be broadcast as
    /// `SteganographicFileEvent`s
    fn load_stego(&mut self, path: PathBuf);
}

impl<'w, 's> LoadStegoCommands for Commands<'w, 's> {
    fn load_stego(&mut self, path: PathBuf) {
        self.add(LoadStegoCommand::new(path));
    }
}
