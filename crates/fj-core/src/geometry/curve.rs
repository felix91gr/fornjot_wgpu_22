use std::collections::BTreeMap;

use crate::{storage::Handle, topology::Surface};

use super::SurfacePath;

/// The geometric definition of a curve
#[derive(Clone, Default)]
pub struct CurveGeom {
    /// # The redundant local definitions of the curve geometry
    ///
    /// ## Implementation Note
    ///
    /// Having multiple redundant definitions is undesirable. However, we can't
    /// just use one global definition in 3D, as we need the local 2D
    /// definitions to approximate and triangulate curves, and we currently
    /// don't have the tools to project a global definition into a local
    /// context.
    ///
    /// Eventually, it should be possible to define the geometry of a curve
    /// once, either locally or globally, and then convert that single
    /// definition into (other) local contexts, as needed. There currently is no
    /// issue to track that specifically, but there is the following issue,
    /// which is a prerequisite for making the required tooling practical:
    ///
    /// <https://github.com/hannobraun/fornjot/issues/2118>
    pub definitions: BTreeMap<Handle<Surface>, LocalCurveGeom>,
}

/// The geometric definition of a curve in 2D surface coordinates
#[derive(Clone)]
pub struct LocalCurveGeom {
    /// The path that defines the curve on its surface
    pub path: SurfacePath,
}