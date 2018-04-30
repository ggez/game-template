//! Example of integrating ggez types with the `warmy` resource loader.

use std::path;

use failure::{self, Fail};
use ggez::{self, audio, graphics};
use warmy;


use error::*;

/// Warmy hands our `load()` method an absolute path, while ggez takes absolute
/// paths into its VFS directory.  Warmy needs to know the real absolute path so
/// it can watch for reloads, so this function strips the path prefix of the warmy
/// Store's root off of the given absolute path and turns it into the style of path
/// that ggez expects.
///
/// Because of this, ggez will have several places that resources *may* live but
/// warmy will only watch for reloads in one of them.  However, that isn't a huge
/// problem: for development you generally want all your assets in one place to
/// begin with, and for deployment you don't need the hotloading functionality.
fn warmy_to_ggez_path(path: &path::Path, root: &path::Path) -> path::PathBuf {
    let stripped_path = path.strip_prefix(root)
        .expect("warmy path is outside of the warmy store?  Should never happen.");
    path::Path::new("/").join(stripped_path)
}

/// Just a test asset that does nothing.
#[derive(Debug, Copy, Clone)]
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
        debug!("Loading test asset: {:?}", key);
        Ok(TestAsset.into())
    }
}

/// A wrapper for a ggez Image, so we can implement warmy's `Load` trait on it.
#[derive(Debug, Clone)]
pub struct Image(pub graphics::Image);
impl warmy::Load<ggez::Context> for Image {
    type Key = warmy::FSKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        store: &mut warmy::Storage<ggez::Context>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        let path = warmy_to_ggez_path(key.as_path(), store.root());
        debug!("Loading image {:?} from file {:?}", path, key.as_path());
        graphics::Image::new(ctx, path)
            .map(|x| warmy::Loaded::from(Image(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}


/// A wrapper for a ggez SoundData, so we can implement warmy's `Load` trait on it.
#[derive(Debug, Clone)]
pub struct SoundData(pub audio::SoundData);
impl warmy::Load<ggez::Context> for SoundData {
    type Key = warmy::FSKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        store: &mut warmy::Storage<ggez::Context>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        let path = warmy_to_ggez_path(key.as_path(), store.root());
        debug!("Loading sound {:?} from file {:?}", path, key.as_path());

        audio::SoundData::new(ctx, path)
            .map(|x| warmy::Loaded::from(SoundData(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}

/// A wrapper for a ggez Font, so we can implement warmy's `Load` trait on it.
///
/// Currently it just forces the font size to 12 pt; we should implement a specific
/// key type for it that includes a font size.
#[derive(Debug, Clone)]
pub struct Font(pub graphics::Font);
impl warmy::Load<ggez::Context> for Font {
    type Key = warmy::FSKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        store: &mut warmy::Storage<ggez::Context>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        let path = warmy_to_ggez_path(key.as_path(), store.root());
        debug!("Loading font {:?} from file {:?}", path, key.as_path());

        graphics::Font::new(ctx, path, 12)
            .map(|x| warmy::Loaded::from(Font(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}
