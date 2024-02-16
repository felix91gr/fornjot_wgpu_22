//! Main entry point to the `fj-core` API
//!
//! See [`Instance`].

use crate::{layers::Layers, validate::ValidationConfig};

/// An instance of the Fornjot core
///
/// This is the main entry point to `fj-core`'s API.
#[derive(Default)]
pub struct Instance {
    /// The layers of data that make up the state of a core instance
    pub layers: Layers,
}

impl Instance {
    /// Construct an instance of `Instance`
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an instance of `Instance`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let layers = Layers::with_validation_config(config);
        Self { layers }
    }
}