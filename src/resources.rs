//! Example of integrating ggez types with the `warmy` resource loader.

use std::path;

use failure::{self, Fail};
use ggez;
use ggez::graphics;
use warmy;


use error::*;

pub struct TestAsset;

impl<C> warmy::Load<C> for TestAsset {
    type Key = warmy::LogicalKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        _ctx: &mut C,
        _store: &mut warmy::Storage<C>
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        debug!("Attempting to load: {:?}", key);
        Ok(TestAsset.into())
    }
}

pub struct Image(pub graphics::Image);
impl warmy::Load<ggez::Context> for Image {
    type Key = warmy::LogicalKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        ctx: &mut ggez::Context,
        _store: &mut warmy::Storage<ggez::Context>,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        debug!("Attempting to load: {:?}", key.as_str());
        let ggez_path = path::Path::new(key.as_str());
        graphics::Image::new(ctx, &ggez_path)
            .map(|x| warmy::Loaded::from(Image(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}
