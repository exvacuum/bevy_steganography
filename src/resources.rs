use bevy::prelude::*;
use occule::Codec;

/// Resource wrapper around a codec
#[derive(Resource, Default)]
pub struct StegoCodec<C>(pub(crate) C)
where
    C: Codec + Send + Sync + 'static;
