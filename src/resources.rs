//! Example of integrating ggez types with the `warmy` resource loader.

use std::path;

use failure::{self, Fail};
use ggez;
use ggez::graphics;
use warmy;


use error::*;

/// A key type for ggez assets.
/// It needs to do a little bit of path hackery
/// to tell warmy where to look when checking for
/// file updates.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GKey(path::PathBuf);

impl GKey {
    pub fn new<P: Into<path::PathBuf>>(path: P) -> Self {
        GKey(path.into())
    }
}

impl warmy::key::Key for GKey {
    fn prepare_key(self, _root: &path::Path) -> Self {
        self
    }
}

impl From<GKey> for warmy::key::DepKey {
    fn from(key: GKey) -> Self {
        warmy::key::DepKey::Path(key.0)
    }
}

/// Just a test asset that does nothing.
pub struct TestAsset;

impl<C> warmy::Load<C> for TestAsset {
    type Key = warmy::key::LogicalKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        _store: &mut warmy::Storage<C>,
        _ctx: &mut C,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        debug!("Attempting to load: {:?}", key);
        Ok(TestAsset.into())
    }
}

/// A ggez Image.
pub struct Image(pub graphics::Image);
impl warmy::Load<ggez::Context> for Image {
    type Key = GKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        _store: &mut warmy::Storage<ggez::Context>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        debug!("Attempting to load: {:?}", key);
        graphics::Image::new(ctx, key.0)
            .map(|x| warmy::Loaded::from(Image(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}
