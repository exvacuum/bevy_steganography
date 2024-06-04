#![warn(missing_docs)]

//! Crate with types to provide a loose integration of steganographic file loading.

use bevy::prelude::*;
use events::SteganographicFileEvent;
use occule::Codec;

pub use occule;
use resources::StegoCodec;

/// Events for this library
pub mod events;

/// Resources for this library
pub mod resources;

/// Commands for this library
pub mod commands;

/// Extension trait for registering a codec tyoe resource and event
pub trait CodecApp {
    /// Register the provided codec instance
    fn register_steganography_codec<'a, C>(&mut self, codec: C) -> &mut Self
    where
        C: Codec + Send + Sync + 'static;

    /// Initialize and register the given codec tyoe
    fn init_steganography_codec<'a, C>(&mut self) -> &mut Self
    where
        C: Codec + Send + Sync + FromWorld + 'static;
}

impl CodecApp for App {
    fn register_steganography_codec<'a, C>(&mut self, codec: C) -> &mut Self
    where
        C: Codec + Send + Sync + 'static,
    {
        self.insert_resource(StegoCodec(codec))
            .add_event::<SteganographicFileEvent<C>>()
    }

    fn init_steganography_codec<'a, C>(&mut self) -> &mut Self
    where
        C: Codec + Send + Sync + FromWorld + 'static,
    {
        let codec = C::from_world(&mut self.world);
        self.register_steganography_codec(codec)
    }
}
