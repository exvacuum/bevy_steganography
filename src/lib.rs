#![warn(missing_docs)]

//! Crate with types to provide a loose integration of steganographic file loading.

use bevy::prelude::*;
use events::SteganographicFileEvent;
use occule::Codec;

pub use occule;
use resources::StegoCodecs;

/// Events for this library
pub mod events;

/// Resources for this library
pub mod resources;

/// Commands for this library
pub mod commands;

/// Plugin which manages registered steganography codecs
pub struct SteganographyPlugin;

impl Plugin for SteganographyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(StegoCodecs::default())
        .add_event::<SteganographicFileEvent>();
    }
}

/// Extension trait for registering a codec tyoe resource and event
pub trait CodecApp {
    /// Register the provided codec instance
    fn register_steganography_codec<C>(&mut self, extensions: Vec<String>, codec: C) -> &mut Self
    where
        C: Codec + Send + Sync + 'static;

    /// Initialize and register the given codec tyoe
    fn init_steganography_codec<C>(&mut self, extensions: Vec<String>) -> &mut Self
    where
        C: Codec + Send + Sync + FromWorld + 'static;
}

impl CodecApp for App {
    fn register_steganography_codec<C>(&mut self, extensions: Vec<String>, codec: C) -> &mut Self
    where
        C: Codec + Send + Sync + 'static,
    {
        self.world.resource_mut::<StegoCodecs>().0.insert(extensions, Box::new(codec));
        self
    }

    fn init_steganography_codec<C>(&mut self, extensions: Vec<String>) -> &mut Self
    where
        C: Codec + Send + Sync + FromWorld + 'static,
    {
        let codec = C::from_world(&mut self.world);
        self.register_steganography_codec(extensions, codec)
    }
}
