use nalgebra::{Point, SVector};

use crate::geometry::aabb::Aabb;

/// Defines geometry that can be sampled
///
/// The `D` parameter defines the dimensionality of the geometry (typically
/// geometry would be 2- or 3-dimensional).
pub trait Geometry<const D: usize> {
    /// Sample the geometry at the specified point
    ///
    /// Returns a `Sample` value which describes, among other attributes, the
    /// distance of the point from the surface.
    fn sample(&self, point: impl Into<Point<f32, D>>) -> Sample<D>;
}

/// The result of sampling geometry at a specific point
///
/// Returned by [`Geometry::sample`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sample<const D: usize> {
    /// The point at which the geometry was sampled
    pub point: Point<f32, D>,

    /// The minimum distance of the point to the surface
    ///
    /// A positive value indicates that the point is outside of the object, a
    /// negative value indicates that the point is inside. Either way, the
    /// absolute value is equal to the distance of the point to the surface.
    pub distance: f32,
}

// TASK: Document
pub trait Normal<const D: usize> {
    // TASK: Document
    fn normal(&self, point: impl Into<Point<f32, D>>) -> SVector<f32, D>;
}

// TASK: Add blanket implementation of `Normal` for 2D geometry.
// TASK: Add blanket implementation of `Normal` for 3D geometry.

/// Defines a bounding volume that encloses geometry
pub trait BoundingVolume<const D: usize> {
    /// Return the geometry's axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}
