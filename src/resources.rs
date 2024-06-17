use bevy::prelude::*;
use indexmap::IndexMap;
use occule::Codec;

/// Resource wrapper around a codec
#[derive(Resource, Default)]
pub struct StegoCodecs(pub(crate) IndexMap<Vec<String>, Box<dyn Codec + Send + Sync>>);
